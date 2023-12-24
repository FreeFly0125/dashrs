#![allow(unused)]

use dash_rs::{GJFormat, HasRobtopFormat};
use std::{collections::HashMap, fmt::Debug};

pub fn load<'a, T: HasRobtopFormat<'a> + Debug>(input: &'a str) -> T {
    let loaded = T::from_robtop_str(input);

    assert!(loaded.is_ok(), "{:?}", loaded.unwrap_err());

    loaded.unwrap()
}

pub fn load2<'a, T: GJFormat<'a> + Debug>(input: &'a str) -> T {
    let loaded = T::from_gj_str(input);

    assert!(loaded.is_ok(), "{:?}", loaded.unwrap_err());

    loaded.unwrap()
}

pub fn load_processed<'a, T: HasRobtopFormat<'a> + ThunkProcessor + Debug>(input: &'a str) -> T {
    let mut t: T = load(input);
    t.process_all_thunks();
    t
}

pub fn load_processed2<'a, T: GJFormat<'a> + ThunkProcessor + Debug>(input: &'a str) -> T {
    let mut t: T = load2(input);
    t.process_all_thunks();
    t
}

pub fn save<'a, T: HasRobtopFormat<'a> + Debug>(t: &T) -> String {
    let saved = t.to_robtop_string();

    assert!(saved.is_ok(), "{:?}", saved.unwrap_err());

    saved.unwrap()
}

pub fn save2<'a, T: GJFormat<'a> + Debug>(t: &T) -> String {
    let mut saved = Vec::new();
    let res = t.write_gj(&mut saved);

    assert!(res.is_ok(), "{:?}", res.unwrap_err());

    String::from_utf8(saved).unwrap()
}

macro_rules! load_save_roundtrip {
    ($t:ty, $load_from:ident, $expected:ident, $sep:expr, $map_like:expr) => {
        load_save_roundtrip!(load_save_roundtrip, $t, $load_from, $expected, $sep, $map_like);
    };

    ($name:ident, $t:ty, $load_from:ident, $expected:ident, $sep:expr, $map_like:expr) => {
        #[test]
        #[allow(non_snake_case)]
        pub fn $name() {
            use helper::*;

            let _ = env_logger::builder().is_test(true).try_init();

            let loaded: $t = load_processed($load_from);
            assert_eq!(loaded, $expected);
            let saved = save(&loaded);
            assert_eq_robtop(&saved, $load_from, $sep, $map_like);
        }
    };
}

macro_rules! load_save_roundtrip2 {
    ($t:ty, $load_from:ident, $expected:ident, $sep:expr, $map_like:expr) => {
        load_save_roundtrip2!(load_save_roundtrip, $t, $load_from, $expected, $sep, $map_like);
    };

    ($name:ident, $t:ty, $load_from:ident, $expected:ident, $sep:expr, $map_like:expr) => {
        #[test]
        #[allow(non_snake_case)]
        pub fn $name() {
            use helper::*;

            let _ = env_logger::builder().is_test(true).try_init();

            let loaded: $t = load_processed2($load_from);
            assert_eq!(loaded, $expected);
            let saved = save2(&loaded);
            assert_eq_robtop(&saved, $load_from, $sep, $map_like);
        }
    };
}

macro_rules! save_load_roundtrip {
    ($t:ty, $to_save:ident) => {
        save_load_roundtrip!(save_load_roundtrip, $t, $to_save);
    };
    ($name:ident, $t:ty, $to_save:ident) => {
        #[test]
        #[allow(non_snake_case)]
        pub fn $name() {
            use helper::*;

            let _ = env_logger::builder().is_test(true).try_init();

            let saved = save(&$to_save);
            let loaded: $t = load_processed(&saved);
            assert_eq!(loaded, $to_save);
        }
    };
}

macro_rules! save_load_roundtrip2 {
    ($t:ty, $to_save:ident) => {
        save_load_roundtrip2!(save_load_roundtrip, $t, $to_save);
    };
    ($name:ident, $t:ty, $to_save:ident) => {
        #[test]
        #[allow(non_snake_case)]
        pub fn $name() {
            use helper::*;

            let _ = env_logger::builder().is_test(true).try_init();

            let saved = save2(&$to_save);
            let loaded: $t = load_processed2(&saved);
            assert_eq!(loaded, $to_save);
        }
    };
}

pub fn assert_eq_robtop(left: &str, right: &str, sep: &str, map_like: bool) {
    let data_left = collect_fields(left.split(sep), map_like);
    let data_right = collect_fields(right.split(sep), map_like);

    // check if key sets are equal
    let mut keys_left: Vec<_> = data_left.keys().collect();
    let mut keys_right: Vec<_> = data_right.keys().collect();

    keys_left.sort();
    keys_right.sort();

    assert_eq!(keys_left, keys_right, "Key sets differ:");

    for key in data_left.keys() {
        assert_eq!(data_left[key], data_right[key], "Value mismatch at index '{}':", key)
    }
}

pub trait ThunkProcessor {
    fn process_all_thunks(&mut self);
}

const INDICES: [&str; 50] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24",
    "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46",
    "47", "48", "49", "50",
];

// Ad-hoc parser for robtop's data format
fn collect_fields<'a>(mut iter: impl Iterator<Item = &'a str>, map_like: bool) -> HashMap<&'a str, &'a str> {
    let mut index = 0;
    let mut map = HashMap::new();

    while let Some(part) = iter.next() {
        let value = if map_like { iter.next().unwrap() } else { part };
        let index = if map_like {
            part
        } else {
            index += 1;
            INDICES[index - 1] // if we ever get a list-like type with more than 50 fields, add more
                               // things to the above array
        };

        assert!(map.insert(index, value).is_none(), "duplicate field {}", index);
    }

    map
}
