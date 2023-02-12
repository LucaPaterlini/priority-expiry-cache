extern crate ahash;
use ahash::AHashMap;

/// 0.1.1 This code is there just to expose the interface.
/// the implementation is not complete.


pub struct Item{
    pub value: String,
    pub expiry: u32,
    pub priority: u32,
    next_expiry: Option<Box<Item>>,
    next_priority: Option<Box<Item>>,
    prev_expiry: Option<Box<Item>>,
    prev_priority: Option<Box<Item>>,
}

// TODO: check existing implemenation for binary tree and doubly linked list
// TODO: if they can be fit for purpose.
// TODO: add the primitives in the dastrascructures module

pub struct Cache {
    map: AHashMap<String, Item>
};

impl Cache {
    pub fn new(capacity:usize) ->Self{
        assert!(capacity>0);
        Self(AHashMap::with_capacity(capacity))
    }
    pub fn get(&self, key: &str) -> Option<&Item> {
        self.0.get(key)
        // set this item as most recently used on the double linked leaf of the time binary tree
        // set this item as most recently used on the double linked leaf of the priority binary tree
    }
    pub fn set(&mut self, key: &str, value: &str, expiry:u32, priority:u32) {
        self.0.insert(key.to_string(), Item{
            value: value.to_string(),
            expiry,
            priority,
            next_expiry: None,
            next_priority: None,
            prev_expiry: None,
            prev_priority: None,
        });
    }
    pub fn evict(&mut self) {
        println!("evict");
    }
}

