use std::{error::Error, time::Duration};

use headless_chrome::{Browser, LaunchOptions, browser::tab::ModifierKey};

fn main() {
	v_utils::clientside!();

	let config = Creds {
		username: "microgrrr".to_string(),
		password: "Valera05!".to_string(),
	};

	post_tweet(config, "Hello, Twitter from Rust!".to_owned()).unwrap();
}

fn post_tweet(config: Creds, tweet: String) -> Result<(), Box<dyn Error>> {
	let browser = init_browser();
	let tab = browser.new_tab()?;

	// Navigate and wait for the page to load completely
	tab.navigate_to("https://x.com/i/flow/login?input_flow_data=%7B%22requested_variant%22%3A%22eyJteCI6IjIifQ%3D%3D%22%7D")?;
	tab.wait_until_navigated()?;

	// Add a small delay to ensure the page is interactive
	std::thread::sleep(Duration::from_secs(2));

	// Auth
	{
		// Includes a delay before and after to prevent some stupid interactivity-based issues
		let enter = || -> Result<(), Box<dyn Error>> {
			std::thread::sleep(Duration::from_millis(500));
			tab.press_key("Enter")?;
			std::thread::sleep(Duration::from_millis(1000));
			Ok(())
		};

		// Username
		let username_field = tab.wait_for_element(r#"input[autocomplete="username"]"#)?;
		username_field.click()?;
		username_field.focus()?;
		tab.type_str(&config.username)?;
		enter()?;

		// Password
		let password_field = tab.wait_for_element(r#"input[name="password"]"#)?;
		password_field.click()?;
		password_field.focus()?;
		tab.type_str(&config.password)?;
		enter()?;
	}

	tab.navigate_to("https://x.com/compose/post")?;

	// Enter tweet text
	let tweet_area = tab.wait_for_element(r#"div[data-testid="tweetTextarea_0"]"#)?;
	tweet_area.click()?;
	tweet_area.focus()?;
	tab.type_str(&tweet)?;

	tab.press_key_with_modifiers("Enter", Some(&[ModifierKey::Ctrl]))?;

	// Wait to ensure the tweet is posted before closing the browser
	std::thread::sleep(std::time::Duration::from_secs(2));
	// `fire_unload` is set to false to avoid potential issues with unload handlers
	tab.close(false)?;

	Ok(())
}

#[derive(Debug)]
struct Creds {
	username: String,
	password: String,
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
