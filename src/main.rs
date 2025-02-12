use clap::{Parser, Subcommand};
use postx::{AppConfig, FollowArgs, run};
use v_utils::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
	#[arg(long)]
	config: Option<ExpandedPath>,
	/// Twitter username
	#[arg(short, long)]
	username: String,
	/// Twitter password
	#[arg(short, long)]
	password: String,
	/// Telegram bot token
	#[arg(short, long)]
	token: String,
}
#[derive(Subcommand)]
enum Commands {
	Follow(FollowArgs),
}

#[tokio::main]
async fn main() {
	v_utils::clientside!();

	let cli = Cli::parse();
	let conf = {
		let mut conf = match AppConfig::read(cli.config) {
			Ok(conf) => conf,
			Err(e) => {
				info!("Could not read config file ({e})\nExpecting to find all args in cli query");
				AppConfig::default()
			}
		};
		if !cli.username.is_empty() {
			conf.twitter.username = cli.username.clone();
		};
		if !cli.password.is_empty() {
			conf.twitter.password = cli.password.clone();
		};
		if !cli.token.is_empty() {
			conf.telegram.bot_token = cli.token.clone();
		};
		assert!(!conf.twitter.username.is_empty());
		assert!(!conf.twitter.password.is_empty());
		assert!(!conf.telegram.bot_token.is_empty());
		conf
	};

	match cli.command {
		Commands::Follow(args) => {
			run(conf, args).await.unwrap();
		}
	}
}
