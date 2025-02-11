
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
}
#[derive(Subcommand)]
enum Commands {
	Follow(FollowArgs),
}

fn main() {
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
		conf
	};

	match cli.command {
		Commands::Follow(args) => {
			run(conf, args).unwrap();
		}
	}
}
