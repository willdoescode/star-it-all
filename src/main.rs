mod cli;
mod json;
use std::fs;
use cli::Cli;
use clap::Clap;
use json::Users;
use std::path::PathBuf;
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let cli: Cli = Cli::parse();
	let token = match cli.token {
		Some(token) => token,
		None => {
			let path = PathBuf::from(std::env::var("HOME").unwrap())
				.join(".staritrc");

			match fs::read_to_string(path.as_path()) {
				Ok(s) => s.trim().to_string(),
				Err(_) => panic!("No .staritrc found in home directory"),
			}
		}
	};

	get_user_info(cli.user.trim(), token, cli.delete).await?;
	Ok(())
}

async fn get_user_info(user: &str, token: String, delete: bool) -> anyhow::Result<()> {
	let client = reqwest::Client::new();
	let mut reqs = Vec::new();
	for page in 1..31 {
		let user = get_users(&client, user, &token, page);
		reqs.push(user);
	}

	let results = try_join_all(reqs).await?;

	let mut reqs = Vec::new();
	for res in results {
		for u in res {
			let star = star(&client, u.full_name, &token, delete);
			reqs.push(star);
		}
	}

	for r in try_join_all(reqs).await? {
		println!("{}: {}", if delete {"Removed Star"} else {"Starred"}, &r.url().path()["/user/starred/".len()..]);
	}

	Ok(())
}

async fn get_users(client: &reqwest::Client, user: &str, token: &str, page: i32)
	-> anyhow::Result<Users>
{
	Ok(
		client.get(format!("https://api.github.com/users/{}/repos?page={}", user, page))
		.header("Authorization", &format!("token {}", token))
		.header("User-Agent", "terminal")
		.send()
		.await?
		.json::<Users>()
		.await?
	)
}

async fn star(client: &reqwest::Client, repo: String, token: &str, delete: bool)
	-> anyhow::Result<reqwest::Response>
{
	Ok(
		if !delete {
			client.put(format!("https://api.github.com/user/starred/{}", repo))
				.header("Accept", "application/vnd.github.v3+json")
				.header("Authorization", &format!("token {}", token))
				.header("User-Agent", "terminal")
				.send()
				.await?
		} else {
			client.delete(format!("https://api.github.com/user/starred/{}", repo))
				.header("Authorization", &format!("token {}", token))
				.header("User-Agent", "terminal")
				.send()
				.await?
		}
	)
}