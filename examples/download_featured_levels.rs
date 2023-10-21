//! Retries the first page of featured levels, downloading the ones that can be copied in-game
//!
//! For each level that is free to copy, the server resposne is stored in `data/<level_id>.lvl`.
//! For each level that requires a password to copy, the server resposne is stored in
//! `data/<level_id>_<password>.lvl`.
//!
//! WARNING: Robtop has set some aggressive ratelimits on the download level endpoint. Attempting
//! to download too many levels (20 per minute from my testing) in too short a period of time will
//! ratelimit you for one hour (HTTP 429 with retry-after=3600). There is also a further ratelimit
//! that last a full day, hit by downloading approximately 100 levels "too fast" (I am not sure on
//! the details here). Use this script at your own risk.

use dash_rs::{
    model::level::Password,
    request::level::{LevelRequest, LevelRequestType, LevelsRequest},
    response::{parse_download_gj_level_response, parse_get_gj_levels_response},
};
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client, Response,
};
use std::{path::Path, time::Duration};

#[tokio::main]
async fn main() {
    let out_dir = Path::new("data");

    if !out_dir.exists() {
        std::fs::create_dir(out_dir).unwrap();
    }

    let http_client = Client::new();

    // Note: There is a further ratelimit of 130 levels per ??, exceeding which will ban you for a day.
    for page in 1.. {
        let request = LevelsRequest::default().request_type(LevelRequestType::Featured).page(page);

        let response = make_request(&http_client, &request.to_url(), request.to_string()).await;
        let response_text = response.text().await.unwrap();

        let levels = parse_get_gj_levels_response(&response_text).unwrap();

        for level in levels {
            println!("Downloading level {} (ID: {})", level.name, level.level_id);

            let download_request = LevelRequest::new(level.level_id);

            let response = make_request(&http_client, &download_request.to_url(), download_request.to_string()).await;

            let response_text = response.text().await.unwrap();

            let mut level = parse_download_gj_level_response(&response_text).unwrap();

            match level.level_data.password.process() {
                Ok(Password::FreeCopy) => std::fs::write(format!("../data/{}.lvl", level.level_id), &response_text).unwrap(),
                Ok(Password::PasswordCopy(pw)) => std::fs::write(format!("../data/{}_{}.lvl", level.level_id, pw), &response_text).unwrap(),
                _ => println!("Sadly not copyable, skipping!"),
            }

            // RobTop has a rate limit of 20 levels per minute. Exceeding it will results in a `429 TOO MANY
            // REQUESTS` response with `retry-after: 3600`, effectively temp-banning you for an hour
            std::thread::sleep(Duration::from_secs(60 / 20 + 1));
        }
    }
}

async fn make_request(client: &Client, endpoint: &str, data: String) -> Response {
    client.post(endpoint)
        .headers(HeaderMap::new())  // boomlings.com rejects any request with a User-Agent header set, so make sure reqwest doesn't "helpfully" add one
        .body(data)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await
        .unwrap()
}
