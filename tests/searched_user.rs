use dash_rs::model::user::searched::SearchedUser;
use std::borrow::Cow;
use dash_rs::model::user::{Color, IconType};

#[macro_use]
mod helper;

const SEARCHED_MICHIGUN_DATA: &str = "1:Michigun:2:703929:13:149:17:12312:6::9:22:10:15:11:12:14:0:15:2:16:34499:3:61161:8:16:4:997";
const SEARCHED_MICHIGUN: SearchedUser = SearchedUser {
    name: Cow::Borrowed("Michigun"),
    user_id: 703929,
    stars: 61161,
    demons: 997,
    index_6: None,
    creator_points: 16,
    icon_index: 22,
    primary_color: Color::Known(0, 0, 0),
    secondary_color: Color::Known(255, 255, 255),
    secret_coins: 149,
    icon_type: IconType::Cube,
    has_glow: true,
    account_id: 34499,
    user_coins: 12312,
};

impl helper::ThunkProcessor for SearchedUser<'_> {
    fn process_all_thunks(&mut self) {
    }
}

save_load_roundtrip!(SearchedUser, SEARCHED_MICHIGUN);
load_save_roundtrip!(SearchedUser, SEARCHED_MICHIGUN_DATA, SEARCHED_MICHIGUN, ":", true);