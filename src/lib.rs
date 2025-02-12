pub mod config;
use std::{error::Error, time::Duration};

use clap::Args;
use color_eyre::eyre::{Result, bail};
use config::TelegramConf;
pub use config::{AppConfig, TwitterConf};
use futures_concurrency::future::FutureExt as _;
use headless_chrome::{Browser, LaunchOptions, browser::tab::ModifierKey};
use teloxide::prelude::*;
use tokio::sync::mpsc;
use v_utils::prelude::*;

#[derive(Debug, Args, Default)]
pub struct FollowArgs {
	/// Channel @ handles to follow, ex: "channel1,channel2"
	channels: Vec<String>,
}

#[derive(Clone, Debug, Default)]
struct Handle(pub String);
impl Handle {
	fn new(handle: String) -> Self {
		if handle.starts_with('@') { Self(handle) } else { Self(format!("@{}", handle)) }
	}
}

pub async fn run(config: AppConfig, args: FollowArgs) -> Result<()> {
	if args.channels.is_empty() {
		bail!("No channels to follow specified");
	}
	if args.channels.len() > 1 {
		unimplemented!("Couldn't be bothered to properly async, so specify only one channel to follow (may fix later).");
	}
	assert_eq!(args.channels.len(), 1); //dbg

	let (tx, mut rx) = tokio::sync::mpsc::channel(10);
	let arc_tg_conf = Arc::new(config.telegram);
	for channel_name in args.channels {
		tokio::spawn(listen_on_channel(Arc::clone(&arc_tg_conf), Handle::new(channel_name), tx.clone()));
	}

	loop {
		tokio::select! {
		_ = tokio::signal::ctrl_c() => {
			info!("Received Ctrl-C, shutting down gracefully...");
			break;
		}
			msg = rx.recv() => {
				match msg {
					Some(msg) => {
						debug!("Processing message: {:?}", msg);

						if let Some(photo) = msg.photo() {
							warn!("Photo received: {:?}, handling is not yet implemented, IGNORING.", photo);
						}

						if let Some(document) = msg.document() {
							warn!("Document received: {:?}, handling is not yet implemented, IGNORING.", document);
						}

						if let Some(text) = msg.text() {
							info!("posting new tweet:\n```\n{text}\n```");
							if let Err(e) = post_tweet(config.twitter.clone(), text) {
								error!("Failed to post tweet: {}", e);
							}
						} else {
							debug!("Message contained no text");
						}
					}
					None => {
						bail!("Channel closed unexpectedly");
					}
				}
			}
		}
	}
	Ok(())
}

pub fn post_tweet(conf: TwitterConf, tweet: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
	let browser = init_browser();
	let tab = browser.new_tab()?;

	// Navigate to login page and wait for it to load completely
	tab.navigate_to("https://x.com/login")?;
	tab.wait_until_navigated()?;

	// Auth
	{
		// Includes delay to prevent some stupid interactivity-based issues
		let enter = || -> Result<(), Box<dyn Error + Send + Sync>> {
			std::thread::sleep(Duration::from_millis(500));
			tab.press_key("Enter")?;
			std::thread::sleep(Duration::from_millis(1000));
			Ok(())
		};

		// Username
		let username_field = tab.wait_for_element(r#"input[autocomplete="username"]"#)?;
		username_field.click()?;
		username_field.focus()?;
		tab.type_str(&conf.username)?;
		enter()?;

		// Password
		let password_field = tab.wait_for_element(r#"input[name="password"]"#)?;
		password_field.click()?;
		password_field.focus()?;
		tab.type_str(&conf.password)?;
		enter()?;
	}

	// Tweet
	tab.navigate_to("https://x.com/compose/post")?;
	let tweet_area = tab.wait_for_element(r#"div[data-testid="tweetTextarea_0"]"#)?;
	tweet_area.click()?;
	tweet_area.focus()?;
	tab.type_str(&tweet)?;
	tab.press_key_with_modifiers("Enter", Some(&[ModifierKey::Ctrl]))?;

	std::thread::sleep(std::time::Duration::from_secs(2)); // Wait to ensure the tweet is posted before closing the browser
	tab.close(false)?; // `fire_unload` is set to false to avoid potential issues with unload handlers

	Ok(())
}

fn init_browser() -> Browser {
	let launch_options = LaunchOptions {
		#[cfg(debug_assertions)]
		headless: false,
		#[cfg(not(debug_assertions))]
		headless: true,
		enable_logging: true,
		enable_gpu: false,
		sandbox: false,
		#[cfg(debug_assertions)]
		devtools: true,
		#[cfg(not(debug_assertions))]
		devtools: false,
		..Default::default()
	};

	Browser::new(launch_options).unwrap()
}

async fn listen_on_channel(tg_conf: Arc<TelegramConf>, handle: Handle, tx: tokio::sync::mpsc::Sender<Message>) -> Result<()> {
	let bot = Bot::new(tg_conf.bot_token.clone());
	let chat = bot.get_chat(handle.0).await?;
	let channel_id = chat.id;
	info!("Connected to channel with ID: {}", channel_id);

	let handler = Update::filter_channel_post().endpoint(move |msg: Message| {
		let tx_clone = tx.clone();
		async move {
			debug!("appending a message from {} to the channel", msg.sender_chat.clone().unwrap().id);
			if let Err(e) = tx_clone.send(msg.clone()).await {
				error!("Failed to send message to channel: {}", e);
			}
			Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
		}
	});

	Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;

	Ok(())
}
