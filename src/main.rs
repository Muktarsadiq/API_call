use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize,Debug)]
struct User {
    login: String,
    id: u32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    site_admin: bool,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers", 
    owner = "rust-lang-nursery",
    repo = "rust-cookbook");

    println!("{}", request_url);

    let client = reqwest::Client::new();
    let response = client
    .get(&request_url)
    .header(USER_AGENT,"rust web-api-client demo")
    .send().await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
  
}
