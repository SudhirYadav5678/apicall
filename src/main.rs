
use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize,Debug)]
struct User{
    login:String,
    id:u32,

}

#[tokio::main]
async fn main()->Result<(),Error>{
let request_url = format!("http://api.github.com/repos/{owner}/{repo}/stargazers",
owner="sudhirkyada5678",
repo="game");  //Uses the format! macro to interpolate values (owner and repo) into the API URL.
println!("{}",request_url);
let client = reqwest::Client::new();
let response = client
    .get(&request_url)
    .header(USER_AGENT,"rust web clinet")
    .send()
    .await?;

    let users:Vec<User>=response.json().await?;
    println!("{:?}",users);
    Ok({})
}