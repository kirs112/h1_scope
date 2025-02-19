
mod input;
mod programs;
mod scopes;

use reqwest::Client;
use input::Opts;
use crate::programs::{get_programs};
use crate::scopes::get_all_assets;

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::read();

    let client = Client::builder().danger_accept_invalid_certs(true)
        .build().unwrap();

    let resp = client.get("https://api.hackerone.com/v1/hackers/programs/security/structured_scopes")
        .basic_auth(&opts.username, Some(&opts.key)).send().await.unwrap();

    if !resp.status().is_success() {
            panic!("username or apikey Error {}", resp.text().await.unwrap());
    }

    let handler = get_programs(client.clone(), &opts.username,&opts.key).await;
    println!("获取程序总数: {:?}", handler.len());

    get_all_assets(client.clone(),&opts.username,&opts.key,handler).await;

}
