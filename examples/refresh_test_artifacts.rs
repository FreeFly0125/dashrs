use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};

use dash_rs::{
    model::{creator::Creator, level::Level, song::NewgroundsSong},
    request::{
        level::{CompletionFilter, LevelRequest, LevelsRequest, SearchFilters},
        user::{UserRequest, UserSearchRequest},
    },
    response::{parse_download_gj_level_response, parse_get_gj_user_info_response, parse_get_gj_users_response},
    GJFormat,
};
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

    // Comment out the ones you dont want to refresh
    refresh_full_levels(&artifacts_path, &http_client).await;
    refresh_listed_levels(&artifacts_path, &http_client).await;
    refresh_profiles(&artifacts_path, &http_client).await;
    refresh_searched_users(&artifacts_path, &http_client).await;

    std::fs::remove_dir_all(&artifacts_backup).unwrap();
}

async fn refresh_full_levels(artifacts_path: &PathBuf, http_client: &Client) {
    println!("Downloading full levels");

    let levels_path = artifacts_path.join("level");
    let levels_to_download = [
        897837,   // time pressure by aeonair
        11774780, // dark realm by stardust1971
    ];

    for level_id in levels_to_download {
        let request = LevelRequest::new(level_id);
        let response = make_request(http_client, &request.to_url(), request.to_string()).await;
        let response_text = response.text().await.unwrap();
        let level = parse_download_gj_level_response(&response_text).unwrap();

        let level_artifact_path = levels_path.join(level_id.to_string());
        let _ = std::fs::create_dir_all(&level_artifact_path);

        std::fs::write(level_artifact_path.join("raw"), response_text.split('#').next().unwrap()).unwrap();
        dump_deserialized_artifact(level_artifact_path, &level);
    }
}

async fn refresh_listed_levels(artifacts_path: &PathBuf, http_client: &Client) {
    println!("Downloading listed levels");

    let listed_levels_path = artifacts_path.join("listed_level");
    let creators_path = artifacts_path.join("creator");
    let songs_path = artifacts_path.join("song");
    let levels_to_dowload = vec![
        72540,    // demon world, 1.3 (?) level
        11774780, // dark realm , 1.9 level
        23298409, // duelo maestro, 2.0 level
        63355989, // fantasy, 2.1 level
        97598449, // Loco Motive, 2.2 platformer level
    ];

    let request = LevelsRequest::default()
        .search_filters(SearchFilters::default().completion_filter(CompletionFilter::limit_search(levels_to_dowload)));

    let response = make_request(&http_client, &request.to_url(), request.to_string()).await;
    let response_text = response.text().await.unwrap();

    // We'll have to reimplement part of the response parsing here, `parse_get_gj_levels_response` is
    // too ergonomic for what we want
    let mut section_iter = response_text.split('#');
    let raw_levels = section_iter.next().unwrap();
    let raw_creators = section_iter.next().unwrap();
    let raw_songs = section_iter.next().unwrap();

    for raw_level in raw_levels.split('|') {
        let level = Level::<()>::from_gj_str(raw_level).unwrap();

        let level_artifact_path = listed_levels_path.join(level.level_id.to_string());
        let _ = std::fs::create_dir_all(&level_artifact_path);

        std::fs::write(level_artifact_path.join("raw"), &raw_level).unwrap();
        dump_deserialized_artifact(level_artifact_path, &level);
    }

    for raw_creator in raw_creators.split('|') {
        let creator = Creator::from_gj_str(raw_creator).unwrap();

        let creator_artifact_path = creators_path.join(creator.user_id.to_string());
        let _ = std::fs::create_dir_all(&creator_artifact_path);

        std::fs::write(creator_artifact_path.join("raw"), &raw_creator).unwrap();
        dump_deserialized_artifact(creator_artifact_path, &creator);
    }

    for raw_song in raw_songs.split("~:~") {
        let song = NewgroundsSong::from_gj_str(raw_song).unwrap();

        let song_artifact_path = songs_path.join(song.song_id.to_string());
        let _ = std::fs::create_dir_all(&song_artifact_path);

        std::fs::write(song_artifact_path.join("raw"), &raw_song).unwrap();
        dump_deserialized_artifact(song_artifact_path, &song);
    }
}

async fn refresh_profiles(artifacts_path: &PathBuf, http_client: &Client) {
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
}

async fn refresh_searched_users(artifacts_path: &PathBuf, http_client: &Client) {
    println!("Downloading search-listed users");

    let searched_users_path = artifacts_path.join("searched_user");
    let users_to_search = ["stardust1971" /* stardust1971 */];

    for username in users_to_search {
        let request = UserSearchRequest::new(username);
        let response = make_request(&http_client, &request.to_url(), request.to_string()).await;
        let response_text = response.text().await.unwrap();

        let searched_user = parse_get_gj_users_response(&response_text).unwrap();

        let searched_user_artifact_path = searched_users_path.join(username.to_string());
        let _ = std::fs::create_dir_all(&searched_user_artifact_path);

        std::fs::write(searched_user_artifact_path.join("raw"), &response_text.split('#').next().unwrap()).unwrap();
        dump_deserialized_artifact(searched_user_artifact_path, &searched_user);
    }
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
