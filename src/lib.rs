use std::collections::HashMap;

pub struct Cache {
    store: HashMap<u32, u32>,
    computes: u32,
}

impl Cache {
    pub fn new() -> Self {
        return Self {
            store: HashMap::new(),
            computes: 0,
        };
    }
    fn expensive_compute(key: u32) -> u32 {
        return key * key;
    }
}

impl Cache {
    // &mut is contagious upward.
    // Any method that calls a &mut self method must itself take &mut self.
    // So extracting helpers can never launder it away — it propagates to every caller in the chain until something breaks it.
    pub fn get(&mut self, key: u32) -> u32 {
        if let Some(value) = self.store.get(&key) {
            return *value;
        }
        let val = Self::expensive_compute(key);
        self.store.insert(key, val);
        self.compute();

        return val;
    }

    fn compute(&mut self) {
        self.computes += 1;
    }

    pub fn remove(&mut self, key: u32) -> bool {
        if let Some(_) = self.store.remove(&key) {
            return true;
        }

        return false;
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
        // Even though I am only doing a get, a read, I am still forced to declare as mut
        // THIS is the case for bulding in Interior Mutability
        let mut cache = Cache::new();
        assert_eq!(cache.get(3), 9);
        assert_eq!(cache.computes, 1)
    }

    #[test]
    fn compute_stays_same_for_existing_value() {
        let mut cache = Cache::new();
        assert_eq!(cache.get(3), 9);
        assert_eq!(cache.get(3), 9);
        assert_eq!(cache.computes, 1);
    }
}
