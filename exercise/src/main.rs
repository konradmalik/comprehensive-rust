use reqwest::{blocking::Client, Url};
use scraper::{Html, Selector};
use std::{
    collections::HashSet,
    sync::{mpsc, Arc, Mutex},
    thread,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("bad http response: {0}")]
    BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
    println!("Checking {:#}", command.url);
    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    let mut link_urls = Vec::new();
    if !command.extract_links {
        return Ok(link_urls);
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => {
                link_urls.push(link_url);
            }
            Err(err) => {
                println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
            }
        }
    }
    Ok(link_urls)
}

type CrawlResult = Result<Vec<Url>, (Url, Error)>;

fn spawn_workers(
    cmd_recv: mpsc::Receiver<CrawlCommand>,
    result_sndr: mpsc::Sender<CrawlResult>,
    thread_count: u8,
) {
    let cmd_recv_pool = Arc::new(Mutex::new(cmd_recv));

    for _ in 0..thread_count {
        let local_cmd_recv = Arc::clone(&cmd_recv_pool);
        let local_result_sndr = result_sndr.clone();

        thread::spawn(move || loop {
            let client = Client::new();
            let locked_cmd_recv = local_cmd_recv.lock().unwrap();

            let Ok(cmd) = locked_cmd_recv.recv() else {
                break;
            };

            match visit_page(&client, &cmd) {
                Ok(links) => {
                    local_result_sndr.send(Ok(links)).unwrap();
                }
                Err(err) => {
                    local_result_sndr.send(Err((cmd.url, err))).unwrap();
                }
            };
        });
    }
}

fn control_scraping(
    start_url: Url,
    cmd_sndr: mpsc::Sender<CrawlCommand>,
    result_recv: mpsc::Receiver<CrawlResult>,
) -> Vec<(Url, Error)> {
    let mut visited = HashSet::new();
    visited.insert(start_url.as_str().to_owned());

    let initialization = CrawlCommand {
        url: start_url,
        extract_links: true,
    };
    cmd_sndr.send(initialization).unwrap();

    let mut bad_urls = Vec::new();
    // cannot track via recv
    // recv will deadlock waiting for potential new results
    // as result senders will never drop
    let mut pending_urls = 1;
    while pending_urls > 0 {
        pending_urls -= 1;

        let result = result_recv.recv().unwrap();
        match result {
            Err(err) => bad_urls.push(err),
            Ok(urls) => {
                for url in urls {
                    if visited.contains(url.as_str()) {
                        continue;
                    }
                    visited.insert(url.as_str().to_owned());

                    cmd_sndr
                        .send(CrawlCommand {
                            url,
                            extract_links: true,
                        })
                        .unwrap();
                    pending_urls += 1;
                }
            }
        }
    }
    bad_urls
}

fn check_links(start_url: Url) -> Vec<(Url, Error)> {
    let (cmd_sndr, cmd_recv) = mpsc::channel();
    let (result_sndr, result_recv) = mpsc::channel();

    spawn_workers(cmd_recv, result_sndr, 6);
    control_scraping(start_url, cmd_sndr, result_recv)
}

fn main() {
    let start_url = Url::parse("https://www.google.org").unwrap();
    let results = check_links(start_url);

    for (url, err) in results {
        println!("{} is bad with {:#}", url, err);
    }
}
