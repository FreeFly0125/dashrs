use dash_rs::model::creator::Creator;
use std::borrow::Cow;

#[macro_use]
mod helper;

const CREATOR_REGISTERED_DATA: &str = "4170784:Serponge:119741";
const CREATOR_REGISTERED: Creator = Creator {
    user_id: 4170784,
    name: Cow::Borrowed("Serponge"),
    account_id: Some(119741),
};

const CREATOR_UNREGISTERED_DATA: &str = "4170784:Serponge:0";
const CREATOR_UNREGISTERED: Creator = Creator {
    user_id: 4170784,
    name: Cow::Borrowed("Serponge"),
    account_id: None,
};

impl helper::ThunkProcessor for Creator<'_> {
    fn process_all_thunks(&mut self) {}
}

save_load_roundtrip2!(save_load_roundtrip_registered, Creator, CREATOR_REGISTERED);
load_save_roundtrip2!(
    load_save_roundtrip_registered,
    Creator,
    CREATOR_REGISTERED_DATA,
    CREATOR_REGISTERED,
    ":",
    false
);

save_load_roundtrip2!(save_load_roundtrip_unregistered, Creator, CREATOR_UNREGISTERED);
load_save_roundtrip2!(
    load_save_roundtrip_unregistered,
    Creator,
    CREATOR_UNREGISTERED_DATA,
    CREATOR_UNREGISTERED,
    ":",
    false
);
