use serde::{Deserialize, Serialize};

pub type Users = Vec<User>;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
	id: i64,
	node_id: String,
	name: String,
	full_name: String,
	private: bool,
	owner: Owner,
	html_url: String,
	description: Option<String>,
	fork: bool,
	url: String,
	forks_url: String,
	keys_url: String,
	collaborators_url: String,
	teams_url: String,
	hooks_url: String,
	issue_events_url: String,
	events_url: String,
	assignees_url: String,
	branches_url: String,
	tags_url: String,
	blobs_url: String,
	git_tags_url: String,
	git_refs_url: String,
	trees_url: String,
	statuses_url: String,
	languages_url: String,
	stargazers_url: String,
	contributors_url: String,
	subscribers_url: String,
	subscription_url: String,
	commits_url: String,
	git_commits_url: String,
	comments_url: String,
	issue_comment_url: String,
	contents_url: String,
	compare_url: String,
	merges_url: String,
	archive_url: String,
	downloads_url: String,
	issues_url: String,
	pulls_url: String,
	milestones_url: String,
	notifications_url: String,
	labels_url: String,
	releases_url: String,
	deployments_url: String,
	created_at: String,
	updated_at: String,
	pushed_at: String,
	git_url: String,
	ssh_url: String,
	clone_url: String,
	svn_url: String,
	homepage: Option<String>,
	size: i64,
	stargazers_count: i64,
	watchers_count: i64,
	language: Option<String>,
	has_issues: bool,
	has_projects: bool,
	has_downloads: bool,
	has_wiki: bool,
	has_pages: bool,
	forks_count: i64,
	mirror_url: Option<serde_json::Value>,
	archived: bool,
	disabled: bool,
	open_issues_count: i64,
	license: Option<License>,
	forks: i64,
	open_issues: i64,
	watchers: i64,
	default_branch: DefaultBranch,
	permissions: Permissions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct License {
	key: String,
	name: String,
	spdx_id: String,
	url: String,
	node_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
	login: String,
	id: i64,
	node_id: String,
	avatar_url: String,
	gravatar_id: String,
	url: String,
	html_url: String,
	followers_url: String,
	following_url: String,
	gists_url: String,
	starred_url: String,
	subscriptions_url: String,
	organizations_url: String,
	repos_url: String,
	events_url: String,
	received_events_url: String,
	#[serde(rename = "type")]
	owner_type: serde_json::Value,
	site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
	admin: bool,
	push: bool,
	pull: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DefaultBranch {
	#[serde(rename = "main")]
	Main,
	#[serde(rename = "master")]
	Master,
	#[serde(rename = "trunk")]
	Trunk,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
	User,
}
