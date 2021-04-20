use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "William Lane <williamlane923@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cli {
	#[clap(short, long)]
	pub token: Option<String>,

	#[clap(short, long)]
	pub user: String,
}