extern crate priority_expiry_cache;

use priority_expiry_cache::PECache;
#[test]
fn new_cache(){
    let _: PECache<String, String, u32, u32> = PECache::new();
}

#[test]
fn get_missing_key(){
    assert_eq!(None::<String>,PECache::<String, String, u32, u32>::new().get("".to_string()));
}

#[test]
fn evict_from_empty_cache(){
    PECache::<String,String, u32, u32>::new().evict(0);
}

#[test]
fn get_and_set_single_element(){
    let (key, value, expiry, priority) = (
        String::from("key"),
        String::from("value"), 1, 1);
    let mut cache = PECache::new();
    cache.set(key.clone(),value.clone(), expiry, priority);
    assert_eq!(value,cache.get(key).unwrap());
}

#[test]
fn set_2_items_same_key_override(){
    let (key, value, expiry, priority) = (
        String::from("key"),
        String::from("value"), 1, 1);

    let (key1, value1, expiry1, priority1) = (
        String::from("key"),
        String::from("hello"), 2, 2);
    let mut cache = PECache::new();
    cache.set(key.clone(),value.clone(), expiry, priority);
    cache.set(key1.clone(),value1.clone(), expiry1, priority1);
    assert_eq!(value1,cache.get(key).unwrap());
    assert_eq!(1,cache.len());

}


#[test]
fn get_and_set_evict_single_element(){
    let (key, value, expiry, priority) = (
        String::from("key"),
        String::from("value"), 1, 1);
    let mut cache = PECache::new();
    cache.set(key.clone(),value.clone(), expiry, priority);
    cache.evict(2);
    assert_eq!(None,cache.get(key));
}

#[test]
fn get_and_set_not_evict_high_barrier_single_element(){
    let (key, value, expiry, priority) = (
        String::from("key"),
        String::from("value"), 1, 1);
    let mut cache = PECache::new();
    cache.set(key.clone(),value.clone(), expiry, priority);
    assert_eq!(value,cache.get(key).unwrap());
}

#[test]
fn insert_2_elements_evict_get_different_time(){
    let (key, value, expiry, priority) = (
        String::from("key"),
        String::from("value"), 0, 1);
    let (key1, value1, expiry1, priority1) = (
        String::from("key1"),
        String::from("value1"), 2, 2);
    let mut cache = PECache::new();
    cache.set(key.clone(),value.clone(), expiry, priority);
    cache.set(key1.clone(),value1.clone(), expiry1, priority1);
    // check before
    assert_eq!(value,cache.get(key.clone()).unwrap());
    assert_eq!(value1,cache.get(key1.clone()).unwrap());

    cache.evict(1);
    // check after
    assert_eq!(value1,cache.get(key1.clone()).unwrap());
    assert_eq!(None,cache.get(key.clone()));
    // make sure empty
    cache.evict(3);
    assert_eq!(None,cache.get(key.clone()));
    assert_eq!(None,cache.get(key1.clone()));
    // evict on empty
    cache.evict(0);
}


#[test]
fn insert_2_elements_evict_by_priority(){
    let (key, value, expiry, priority) = (
        String::from("z_key"),
        String::from("z_value"), 0, 0);
    let (key1, value1, expiry1, priority1) = (
        String::from("key1"),
        String::from("value1"), 0, 0);
    let mut cache = PECache::new();
    cache.set(key.clone(),value.clone(), expiry, priority);
    cache.set(key1.clone(),value1.clone(), expiry1, priority1);
    // check before
    assert_eq!(value,cache.get(key.clone()).unwrap());
    assert_eq!(value1,cache.get(key1.clone()).unwrap());
    cache.evict(2);
    // check after
    assert_eq!(value.clone(),cache.get(key.clone()).unwrap());
    assert_eq!(None,cache.get(key1.clone()));
    // make sure empty
    cache.evict(0);
    assert_eq!(None,cache.get(key.clone()));
    assert_eq!(None,cache.get(key1.clone()));
    // evict on empty
    cache.evict(0);
}

#[test]
fn eviction_by_lru(){
    let (key, value, expiry, priority) = (
        String::from("z_key"),
        String::from("z_value"), 10, 2);
    let (key1, value1, expiry1, priority1) = (
        String::from("key1"),
        String::from("value1"), 11, 2);
    let (key2, value2, expiry2, priority2) = (
        String::from("key2"),
        String::from("value2"), 12, 2);

    let mut cache = PECache::new();
    cache.set(key.clone(),value.clone(), expiry, priority);
    cache.set(key1.clone(),value1.clone(), expiry1, priority1);
    cache.set(key2.clone(),value2.clone(), expiry2, priority2);

    cache.get(key.clone());
    cache.evict(5);
    // deleted in the middle
    assert_eq!(value,cache.get(key.clone()).unwrap());
    assert_eq!(None,cache.get(key1.clone()));
    assert_eq!(value2,cache.get(key2.clone()).unwrap());
}

#[test]
fn get_and_set_single_element_generic(){
    #[derive(Clone,Hash,Ord,Eq,PartialOrd, PartialEq)]
    struct KeyGeneric(u16, String);
    #[derive(Clone,Hash,Ord,Eq,PartialOrd, PartialEq, Debug)]
    struct ValueGeneric(u16, String,u32);

    let (key, value, expiry, priority) = (
        KeyGeneric{ 0: 10, 1: "a string".to_string()},
        ValueGeneric{0:2,1:"hello".to_string(),2:2},
        1,
        0
    );
    let mut cache = PECache::new();
    cache.set(key.clone(),value.clone(), expiry, priority);
    assert_eq!(value,cache.get(key).unwrap());
}