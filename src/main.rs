mod cli;
mod json;
use cli::Cli;
use json::Users;
use clap::Clap;
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let cli: Cli = Cli::parse();
	let token = match cli.token {
		Some(token) => token,
		None => {
			let path = format!("{}/.staritrc", std::env::var("HOME").unwrap());
			match fs::read_to_string(&path[..]).await {
				Ok(s) => s.trim().to_string(),
				Err(_) => panic!("No .staritrc found in home directory"),
			}
		}
	};

	get_user_info(cli.user, token, 1).await?;
	Ok(())
}

async fn get_user_info(user: String, token: String, page: i32) -> anyhow::Result<()> {
	let client = reqwest::Client::new();
	let x = client.get(format!("https://api.github.com/users/{}/repos?page={}", user, page))
		.header("Authorization", &format!("token {}", token)[..])
		.header("User-Agent", "terminal")
		.send()
		.await?
		.json::<Users>()
		.await?;

	println!("{:?}", x);

	Ok(())
}
