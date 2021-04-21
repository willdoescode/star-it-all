use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "v0.1.2", author = "William Lane <williamlane923@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cli {
	/// Enter your token here or add it to ~/.staritrc
	#[clap(short, long)]
	pub token: Option<String>,

	/// Username for user to star all repos for
	#[clap(short, long)]
	pub user: String,

	/// Instead of starring all repos remove stars on all repos
	#[clap(short, long)]
	pub delete: bool,
}