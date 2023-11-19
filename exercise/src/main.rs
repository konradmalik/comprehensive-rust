use anyhow::Result;
use futures::{future, join};
use reqwest;
use std::{collections::HashMap, time::Duration};

async fn size_of_page(url: &str) -> Result<usize> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())
}

async fn just_sleep() -> Result<()> {
    Ok(tokio::time::sleep(Duration::from_secs(2)).await)
}

#[tokio::main]
async fn main() {
    let urls: [&str; 4] = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
        "BAD_URL",
    ];
    let futures_iter = urls.into_iter().map(size_of_page);
    let results = join!(future::join_all(futures_iter), just_sleep());
    let page_sizes_dict: HashMap<&str, Result<usize>> =
        urls.into_iter().zip(results.0.into_iter()).collect();
    println!("slept {:?}", results.1);
    println!("{:?}", page_sizes_dict);
}
