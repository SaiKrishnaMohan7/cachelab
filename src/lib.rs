use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

pub struct Cache {
    store: RefCell<HashMap<u32, u32>>,
    computes: Cell<u32>,
}

impl Cache {
    pub fn new() -> Self {
        return Self {
            store: RefCell::new(HashMap::new()),
            computes: Cell::new(0),
        };
    }
    fn expensive_compute(key: u32) -> u32 {
        return key * key;
    }
}

impl Cache {
    pub fn get(&self, key: u32) -> u32 {
        if let Some(value) = self.store.borrow().get(&key) {
            return *value;
        }
        let val = Self::expensive_compute(key);
        self.store.borrow_mut().insert(key, val);
        self.compute();

        return val;
    }

    fn compute(&self) {
        self.computes.set(self.computes.get() + 1);
    }

    pub fn remove(&self, key: u32) -> bool {
        if let Some(_) = self.store.borrow_mut().remove(&key) {
            return true;
        }

        return false;
    }

    // to be used by integration test later on
    pub fn get_compute(&self) -> u32 {
        return self.computes.get();
    }
}

// cfg gates the module
// will be present in bin and only run during cargo test
#[cfg(test)]
// a child module.
// Child modules can see the parent's private items, which is why cache.count works without a getter.
mod tests {
    // pull everything from the parent module (Cache, etc.) into scope.
    use super::*;

    // Mark a fn as a runnable test
    #[test]
    fn computes_on_miss() {
        let cache = Cache::new();
        assert_eq!(cache.get(3), 9);
        assert_eq!(cache.computes.get(), 1);
    }

    #[test]
    fn compute_stays_same_for_existing_value() {
        let cache = Cache::new();
        assert_eq!(cache.get(3), 9);
        assert_eq!(cache.get(3), 9);
        assert_eq!(cache.computes.get(), 1);
    }
}
