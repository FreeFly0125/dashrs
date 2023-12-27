#![allow(dead_code)] // not all test modules use all functions

use std::{
    collections::BTreeMap,
    fmt::Debug,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use dash_rs::{GJFormat, IndexedDeserializer};
use pretty_assertions::assert_eq;
use serde::Deserialize;

pub fn load_test_units<D>(unit_dir: impl AsRef<Path>) -> BTreeMap<PathBuf, TestUnit<D>> {
    let mut map = BTreeMap::new();

    for dir_entry in std::fs::read_dir(unit_dir).unwrap() {
        let dir_entry = dir_entry.unwrap();

        assert!(dir_entry.metadata().unwrap().is_dir());

        map.insert(dir_entry.path(), TestUnit::new(&dir_entry.path()));
    }

    map
}

pub trait Testable {
    type Target<'a>: GJFormat<'a> + Deserialize<'a> + Debug + for<'b> PartialEq<Self::Target<'b>>;

    /// Canonicalizes this test target object before comparisons
    ///
    /// For example, this is where all Thunks should be evaluated
    fn canonicalize(_target: &mut Self::Target<'_>) {}
}

pub struct TestUnit<D> {
    raw: PathBuf,
    processed: PathBuf,
    ghost: PhantomData<D>,
}

impl<D> TestUnit<D> {
    fn new(unit_container: impl AsRef<Path>) -> Self {
        let raw = unit_container.as_ref().join("raw");
        let processed = unit_container.as_ref().join("processed");

        assert!(raw.exists());
        assert!(processed.exists());

        TestUnit {
            raw,
            processed,
            ghost: PhantomData,
        }
    }

    fn load_raw_data(&self) -> String {
        let raw_data = std::fs::read(&self.raw).unwrap();
        String::from_utf8(raw_data).unwrap()
    }
}

impl<D> TestUnit<D>
where
    D: Testable,
{
    pub fn test_consistency(&self) {
        let processed_json = std::fs::read(&self.processed).unwrap();
        let processed_json = std::str::from_utf8(&processed_json).unwrap();
        let processed_artifact: D::Target<'_> = serde_json::from_str(processed_json).unwrap();

        let raw = self.load_raw_data();
        let mut processed = D::Target::from_gj_str(&raw).unwrap();
        D::canonicalize(&mut processed);

        assert_eq!(processed_artifact, processed);
    }

    pub fn test_load_save_roundtrip(&self) {
        let raw = self.load_raw_data();
        let mut loaded = D::Target::from_gj_str(&raw).unwrap();
        D::canonicalize(&mut loaded);

        let mut buffer = Vec::new();
        loaded.write_gj(&mut buffer).unwrap();
        let saved = std::str::from_utf8(&buffer).unwrap();

        assert_indexed_strings_equal::<D::Target<'static>>(&raw, saved)
    }

    pub fn test_save_load_roundtrip(&self) {
        let processed_json = std::fs::read(&self.processed).unwrap();
        let processed_json = std::str::from_utf8(&processed_json).unwrap();
        let processed: D::Target<'_> = serde_json::from_str(processed_json).unwrap();

        let mut buffer = Vec::new();
        processed.write_gj(&mut buffer).unwrap();
        let saved = std::str::from_utf8(&buffer).unwrap();

        let mut restored = D::Target::from_gj_str(saved).unwrap();
        D::canonicalize(&mut restored);

        assert_eq!(processed, restored);
    }
}

fn assert_indexed_strings_equal<'a, D: GJFormat<'a>>(a: &str, b: &str) {
    let mut deserializer_a = IndexedDeserializer::new(a, D::DELIMITER, D::MAP_LIKE);
    let mut deserializer_b = IndexedDeserializer::new(b, D::DELIMITER, D::MAP_LIKE);

    if D::MAP_LIKE {
        let map_a = BTreeMap::<&str, &str>::deserialize(&mut deserializer_a).unwrap();
        let map_b = BTreeMap::<&str, &str>::deserialize(&mut deserializer_b).unwrap();

        // BTreeMap + pretty_assertions will make sure that this is easily interpretable
        assert_eq!(map_a, map_b);
    } else {
        let vec_a = Vec::<&str>::deserialize(&mut deserializer_a).unwrap();
        let vec_b = Vec::<&str>::deserialize(&mut deserializer_b).unwrap();

        assert_eq!(vec_a, vec_b);
    }
}
