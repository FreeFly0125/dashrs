use std::{fs::OpenOptions, path::Path};

use dash_rs::{request::{level::LevelRequest, user::UserRequest}, response::{parse_download_gj_level_response, parse_get_gj_user_info_response}};
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client, Response,
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    // We want debug logging so that it tells us about unmapped indices
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    let http_client = Client::new();

    let artifacts_path = Path::new("tests").join("artifacts");
    let artifacts_backup = Path::new("tests").join("artifacts_backup");

    std::fs::rename(&artifacts_path, &artifacts_backup).unwrap();

    println!("Downloading levels");

    let levels_path = artifacts_path.join("level");
    let levels_to_download = [897837 /* time pressure by aeonair */, 11774780 /* dark realm by stardust1971 */];

    for level_id in levels_to_download {
        let request = LevelRequest::new(level_id);
        let response = make_request(&http_client, &request.to_url(), request.to_string()).await;
        let response_text = response.text().await.unwrap();
        let level = parse_download_gj_level_response(&response_text).unwrap();

        let level_artifact_path = levels_path.join(level_id.to_string());
        let _ = std::fs::create_dir_all(&level_artifact_path);

        std::fs::write(level_artifact_path.join("raw"), response_text.split('#').next().unwrap()).unwrap();
        dump_deserialized_artifact(level_artifact_path, &level);
    }

    println!("Downloading profiles");

    let profiles_path = artifacts_path.join("profile");
    let profiles_to_download = [8451 /* stardust1971 */];

    for account_id in profiles_to_download {
        let request = UserRequest::new(account_id);
        let response = make_request(&http_client, &request.to_url(), request.to_string()).await;
        let response_text = response.text().await.unwrap();

        let profile = parse_get_gj_user_info_response(&response_text).unwrap();

        let profile_artifact_path = profiles_path.join(account_id.to_string());
        let _ = std::fs::create_dir_all(&profile_artifact_path);

        std::fs::write(profile_artifact_path.join("raw"), &response_text).unwrap();
        dump_deserialized_artifact(profile_artifact_path, &profile);
    }

    std::fs::remove_dir_all(&artifacts_backup).unwrap();
}

fn dump_deserialized_artifact<S: Serialize>(dest: impl AsRef<Path>, artifact: &S) {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(dest.as_ref().join("processed"))
        .unwrap();
    serde_json::to_writer_pretty(&mut file, artifact).unwrap();
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
