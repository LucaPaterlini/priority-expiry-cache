extern crate lru;

use std::collections::{BTreeMap, BTreeSet,HashMap};
use std::hash::Hash;
use lru::LruCache;

struct Page<K,V,E,P> {
    key: K,
    value: V,
    expiry: E,
    priority: P,
}

#[derive(Ord,PartialOrd, PartialEq, Eq)]
struct ItemExpiry<E,K> {
    expiry: E,
    key: K,
}

pub struct PECache<K,V,E,P> {
    access_map: HashMap<K, Page<K,V,E,P>>,
    evict_expiry: BTreeSet<ItemExpiry<E,K>>,
    evict_priority: BTreeMap<P, LruCache<K, bool>>,
}

impl <K:Clone+Hash+Ord,
    V: Clone+Eq+Hash+PartialEq,
    E: Clone + Ord + Eq,
    P: Clone + Ord
>PECache<K,V,E,P> {
    /// Creates a new PE Cache.
    ///
    /// # Examples
    /// ```
    /// use priority_expiry_cache::PECache;
    /// let mut new_cache:PECache<String,String,u32,u32>= PECache::new();
    /// ```
    pub fn new() -> Self {
        Self {
            access_map: Default::default(),
            evict_expiry: Default::default(),
            evict_priority: Default::default(),
        }
    }

    /// Add a new item to the cache or override the existing one if present in O(1) time.
    ///
    /// # Examples
    /// ```
    /// use priority_expiry_cache::PECache;
    /// let mut new_cache:PECache<String,String,u32,u32>= PECache::new();
    ///     let (key, value, expiry, priority) = (
    ///         String::from("key"),
    ///         String::from("value"), 1, 1);
    /// new_cache.set(key.clone(),value.clone(), expiry, priority);
    ///
    /// ```
    pub fn set(&mut self, key: K, value: V, expiry: E, priority: P) {
        // addition to the btree for time
        let key_expiry: ItemExpiry<E, K> = ItemExpiry {
            expiry: expiry.clone(),
            key: key.clone(),
        };
        self.evict_expiry.insert(key_expiry);
        // addition to the btree for priority
        self.evict_priority.entry(priority.clone())
            .or_insert(LruCache::unbounded()).push(key.clone(), true);
        // add to the map
        let page = Page { key: key.clone(), value, expiry, priority };
        self.access_map.insert(key, page);
        return;
    }
    /// Gat the value associated with the key if present or None if not in O(1) time.
    ///
    /// # Examples
    /// ```
    /// use priority_expiry_cache::PECache;
    /// let mut new_cache:PECache<String,String,u32,u32>= PECache::new();
    ///
    /// // the get operation
    /// let extracted_value = new_cache.get("key".to_string());
    /// ```
    pub fn get(&mut self, key: K) -> Option<V> {
        if let Some(page) = self.access_map.get(&key) {
            // change the order in the last recently data structure
            self.evict_priority.get_mut(&page.priority).unwrap().promote(&page.key);
            return Some(page.value.clone());
        }
        None
    }
    /// Evict 1 element following this policy if at least one element is present in O(1).
    ///
    /// Policy:
    /// - If an expired item is available. Remove it. If multiple items have the same expiry, removing any one suffices.
    /// - If condition #1 can’t be satisfied, remove an item with the least priority.
    /// - If more than one item satisfies condition #2, remove the least recently used one.
    /// - Multiple items can have the same priority and expiry.
    ///
    /// # Examples
    /// ```
    /// use priority_expiry_cache::PECache;
    /// let mut new_cache:PECache<String,String,u32,u32>= PECache::new();
    ///
    ///
    /// let extracted_value = new_cache.evict(10);
    /// ```
    pub fn evict(&mut self, barrier: E) {
        if self.access_map.len() == 0 {
            return;
        }
        // get key by check firs by time and then by priority/lru
        let target_item = self.evict_expiry.first().unwrap();

        let key_target = if target_item.expiry <= barrier { target_item.key.clone() } else {
            let target_item = self.evict_priority.first_entry().unwrap();
            let (key,_) = target_item.get().peek_lru().unwrap();
            key.clone()
        };
        // delete from the map
        let page = self.access_map.remove(&key_target).unwrap();
        // delete from the expiry tree
        self.evict_expiry.remove(&ItemExpiry{expiry: page.expiry.clone(),key: page.key.clone()});
        // delete from priority tree
        let node_priority = self.evict_priority.get_mut(&page.priority).unwrap();
        node_priority.pop(&page.key);
        // delete the node if empty after the item removal
        if node_priority.len() == 0 {
            self.evict_priority.remove(&page.priority);
        }
        return
    }
    /// Return the Length of Cache items in O(1).
    ///
    /// # Examples
    /// ```
    /// use priority_expiry_cache::PECache;
    /// let mut new_cache:PECache<String,String,u32,u32>= PECache::new();
    ///
    ///
    /// let cache_n_size = new_cache.len();
    /// ```

    pub fn len(&self)->usize{
        self.access_map.len()
    }
}