#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

struct LRUCache {
    map: HashMap<i32, i32>,
    deque: VecDeque<i32>,
    capacity: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            deque: VecDeque::with_capacity(capacity as usize),
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.map.get(&key) {
            self.deque.retain(|&k| k != key);
            self.deque.push_front(key);

            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.deque.retain(|&k| k != key);

        if self.at_capacity() {
            if let Some(lru) = self.deque.pop_back() {
                self.map.remove(&lru);
            }
        }

        self.map.insert(key, value);
        self.deque.push_front(key);
    }

    fn at_capacity(&self) -> bool {
        self.deque.len() == self.capacity as usize
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    pub fn test_lru_cache_flow1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);

        let mut result = lru_cache.get(1);

        assert_eq!(result, 1, "Expected: {}, but Got: {}", 1, result);

        lru_cache.put(3, 3);

        result = lru_cache.get(2);

        assert_eq!(result, -1, "Expected: {}, but Got: {}", -1, result);
    }
}
