pub mod config;
use std::{error::Error, time::Duration};

use clap::Args;
use color_eyre::eyre::{Result, eyre};
pub use config::{AppConfig, TwitterConf};
use headless_chrome::{Browser, LaunchOptions, browser::tab::ModifierKey};

#[derive(Debug, Args, Default)]
pub struct FollowArgs {
	/// Channel @ handles to follow, ex: "channel1,channel2"
	channels: Vec<String>,
}

pub fn run(config: AppConfig, args: FollowArgs) -> Result<()> {
	let tweet = "Hello, world!"; //dbg
	post_tweet(config.twitter, tweet.to_string()).map_err(|e| eyre!(e))?;
	Ok(())
}

pub fn post_tweet(conf: TwitterConf, tweet: String) -> Result<(), Box<dyn Error + Send + Sync>> {
	let browser = init_browser();
	let tab = browser.new_tab()?;

	// Navigate and wait for the page to load completely
	tab.navigate_to("https://x.com/i/flow/login?input_flow_data=%7B%22requested_variant%22%3A%22eyJteCI6IjIifQ%3D%3D%22%7D")?;
	tab.wait_until_navigated()?;
	std::thread::sleep(Duration::from_millis(200)); // wait until page is interactive

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
		headless: false, //dbg: temporarily run in graphical
		enable_logging: true,
		enable_gpu: false,
		sandbox: false,
		devtools: true,
		..Default::default()
	};

	Browser::new(launch_options).unwrap()
}
