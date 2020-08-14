use std::collections::HashMap;
use dash_rs::HasRobtopFormat;
use std::fmt::Debug;

pub fn load<'a, T: HasRobtopFormat<'a> + Debug>(input: &'a str) -> T {
    let loaded = T::from_robtop_str(input);

    assert!(loaded.is_ok(), "{:?}", loaded.unwrap_err());

    loaded.unwrap()
}

pub fn save<'a, T: HasRobtopFormat<'a> + Debug>(t: &T) -> String {
    let saved = t.to_robtop_string();

    assert!(saved.is_ok(), "{:?}", saved.unwrap_err());

    saved.unwrap()
}

macro_rules! load_save_roundtrip {
    ($t: ty, $load_from: ident, $expected: ident, $sep: expr, $map_like: expr)  => {
        load_save_roundtrip!(load_save_roundtrip, $t, $load_from, $expected, $sep, $map_like);
    };

    ($name: ident, $t: ty, $load_from: ident, $expected: ident, $sep: expr, $map_like: expr) => {
        #[test]
        pub fn $name() {
            use helper::*;

            let _ = env_logger::builder().is_test(true).try_init();

            let mut loaded: $t = load($load_from);
            loaded.process_all_thunks();
            assert_eq!(loaded, $expected);
            let saved = save(&loaded);
            assert_eq_robtop(&saved, $load_from, $sep, $map_like);
        }
    };
}

macro_rules! save_load_roundtrip {
    ($t: ty, $to_save: ident) => {
        save_load_roundtrip!(save_load_roundtrip, $t, $to_save);
    };
    ($name: ident, $t: ty, $to_save: ident) => {
        #[test]
        pub fn $name() {
            use helper::*;

            let _ = env_logger::builder().is_test(true).try_init();

            let saved = save(&$to_save);
            let mut loaded: $t = load(&saved);
            loaded.process_all_thunks();
            assert_eq!(loaded, $to_save);
        }
    };
}


pub fn assert_eq_robtop(left: &str, right: &str, sep: &str, map_like: bool) {
    let data_left = collect_fields(left.split(sep), map_like);
    let data_right = collect_fields(right.split(sep), map_like);

    // check if key sets are equal
    assert_eq!(data_left.keys().collect::<Vec<_>>().sort(), data_right.keys().collect::<Vec<_>>().sort(), "Key sets differ:");

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
fn collect_fields<'a>(mut iter: impl Iterator<Item=&'a str>, map_like: bool) -> HashMap<&'a str, &'a str> {
    let mut index = 0;
    let mut map = HashMap::new();

    while let Some(part) = iter.next() {
        let value = if map_like { iter.next().unwrap() } else { part };
        let index = if map_like { part } else {
            index += 1;
            INDICES[index - 1] // if we ever get a list-like type with more than 50 fields, add more things to the above array
        };

        assert!(map.insert(index, value).is_none(), "duplicate field {}", index);
    }

    map
}
