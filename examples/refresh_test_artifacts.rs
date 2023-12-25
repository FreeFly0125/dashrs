use dash_rs::request::level::LevelRequest;
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client, Response,
};

#[tokio::main]
async fn main() {
    let http_client = Client::new();

    let download_time_pressure = LevelRequest::new(897837);
    let response = make_request(&http_client, &download_time_pressure.to_url(), download_time_pressure.to_string()).await;
    std::fs::write(
        "tests/data/897837_time_pressure_gjdownload_response",
        &response.text().await.unwrap().split('#').next().unwrap(),
    )
    .unwrap();

    let download_dark_realm = LevelRequest::new(11774780);
    let response = make_request(&http_client, &download_dark_realm.to_url(), download_dark_realm.to_string()).await;
    std::fs::write(
        "tests/data/11774780_dark_realm_gjdownload_response",
        &response.text().await.unwrap().split('#').next().unwrap(),
    )
    .unwrap();
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
