mod cli;
mod json;
use cli::Cli;
use json::Users;
use clap::Clap;
use tokio::fs;
use futures::future::try_join_all;

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
	get_user_info("aidenybai", token, cli.delete).await?;
	get_user_info(cli.user.trim().to_string(), token, cli.delete).await?;
	Ok(())
}

async fn get_user_info(user: String, token: String, delete: bool) -> anyhow::Result<()> {
	let client = reqwest::Client::new();
	let mut reqs = Vec::new();
	for page in 1..31 {
		let user = get_users(&client, &user, &token, page);
		reqs.push(user);
	}

	let results = try_join_all(reqs).await?;
	println!("{:?}", results);

	let mut reqs = Vec::new();
	for res in results {
		for u in res {
			let star = star(&client, u.full_name, &token, delete);
			reqs.push(star);
		}
	}

	let x = try_join_all(reqs).await?;
	println!("{:?}", x);

	Ok(())
}

async fn get_users(client: &reqwest::Client, user: &String, token: &String, page: i32)
	-> anyhow::Result<Users>
{
	Ok(
		client.get(format!("https://api.github.com/users/{}/repos?page={}", user, page))
		.header("Authorization", &format!("token {}", token)[..])
		.header("User-Agent", "terminal")
		.send()
		.await?
		.json::<Users>()
		.await?
	)
}

async fn star(client: &reqwest::Client, repo: String, token: &String, delete: bool)
	-> anyhow::Result<reqwest::Response>
{
	Ok(
		if !delete {
			client.put(format!("https://api.github.com/user/starred/{}", repo))
				.header("Accept", "application/vnd.github.v3+json")
				.header("Authorization", &format!("token {}", token)[..])
				.header("User-Agent", "terminal")
				.send()
				.await?
		} else {
			client.delete(format!("https://api.github.com/user/starred/{}", repo))
				.header("Authorization", &format!("token {}", token)[..])
				.header("User-Agent", "terminal")
				.send()
				.await?
		}
	)
}
