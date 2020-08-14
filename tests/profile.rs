use dash_rs::model::user::profile::{Profile, Youtube, Twitter};
use std::borrow::Cow;
use dash_rs::model::user::{ModLevel, Color};

#[macro_use]
mod helper;

const PROFILE_STARDUST1971_DATA: &str = "1:stardust1971:2:2073761:13:149:17:498:10:9:11:10:3:13723:46:2312:4:484:8:19:18:0:19:0:50:0:20:\
                                         stardust19710:21:95:22:48:23:33:24:18:25:11:26:10:28:1:43:2:48:13:30:0:16:8451:31:0:44:\
                                         stadust1971:45::49:0:38:0:39:579:40:0:29:1";

const PROFILE_STARDUST1971: Profile = Profile {
    name: Cow::Borrowed("stardust1971"),
    user_id: 2073761,
    stars: 13723,
    demons: 484,
    creator_points: 19,
    primary_color: Color::Known(255, 0, 0),
    secondary_color: Color::Known(255, 125, 0),
    secret_coins: 149,
    account_id: 8451,
    user_coins: 498,
    index_18: Cow::Borrowed("0"),
    index_19: Cow::Borrowed("0"),
    youtube_url: Some(Youtube(Cow::Borrowed("stardust19710"))),
    cube_index: 95,
    ship_index: 48,
    ball_index: 33,
    ufo_index: 18,
    wave_index: 11,
    robot_index: 10,
    has_glow: true,
    index_29: Cow::Borrowed("1"),
    global_rank: Some(0),
    index_31: Cow::Borrowed("0"),
    spider_index: 2,
    twitter_url: Some(Twitter(Cow::Borrowed("stadust1971"))),
    twitch_url: None,
    diamonds: 2312,
    death_effect_index: 13,
    mod_level: ModLevel::None,
    index_50: Cow::Borrowed("0"),
};


impl helper::ThunkProcessor for Profile<'_> {
    fn process_all_thunks(&mut self) {

    }
}

save_load_roundtrip!(Profile, PROFILE_STARDUST1971);
load_save_roundtrip!(Profile, PROFILE_STARDUST1971_DATA, PROFILE_STARDUST1971, ":", true);