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
			println!(
				"{}: {}https://github.com/{}/{}{}",
				if delete {"Removed Star"} else {"Starred"},
				termion::color::Fg(termion::color::LightGreen),
				user,
				u.name,
				termion::color::Fg(termion::color::Reset)
			);
			println!(
				"  Language: {}{}{}",
				termion::color::Fg(termion::color::Yellow),
				if u.language.is_some() {u.language.unwrap()} else {"None".to_string()},
				termion::color::Fg(termion::color::Reset),
			);
			println!(
				"  Stargazers: {}{}{}",
				termion::color::Fg(termion::color::Red),
			  u.stargazers_count + if delete {-1} else {1},
				termion::color::Fg(termion::color::Reset)
			);
			println!(
				"  Watchers: {}{}{}\n",
				termion::color::Fg(termion::color::Green),
				u.watchers_count,
				termion::color::Fg(termion::color::Reset)
			);
			reqs.push(star);
		}
	}

	try_join_all(reqs).await?;

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