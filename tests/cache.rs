extern crate priority_expiry_cache;
use priority_expiry_cache::Cache;

#[test]
fn new_cache(){
    let _ = Cache::new(1);
}

#[test]
fn add_and_collect_one_element(){
    let (key, value, expiry, priority) = ("key", "value", 1, 1);
    let mut cache = Cache::new(1);
    cache.set(key, value, expiry, priority);
    assert_eq!(cache.get(key).unwrap().value, value);
}