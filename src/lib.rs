use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

pub struct Entry {
    key: u32,
    value: u32,
}

impl Drop for Entry {
    fn drop(&mut self) {
        println!("Entry Dropped for key: {}", self.key);
    }
}
pub struct Cache {
    store: RefCell<HashMap<u32, Entry>>,
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
        // `&` in a pattern STRIPS a reference layer — same operation as `*` in an expression.
        // Both are gated by Copy: stripping has to produce an owned value, and for a
        // non-Copy type the only way to do that is to MOVE it out — illegal through a `&`.
        //
        //   HashMap<u32, u32>   -> Some(&v) works: u32 is Copy, stripping duplicates it.
        //   HashMap<u32, Entry> -> Some(&e) FAILS: Entry is not Copy, stripping would move
        //                          it out of the map, which we only have a `&` to.
        //
        // So: don't strip. Bind the reference, read the Copy field through it, move nothing.
        if let Some(entry) = self.store.borrow().get(&key) {
            return entry.value; // entry: &Entry — auto-deref, copies out just the u32 field
        }
        let val = Self::expensive_compute(key);
        self.store
            .borrow_mut()
            .insert(key, Entry { key, value: val });
        self.compute();

        return val;
    }

    fn compute(&self) {
        self.computes.set(self.computes.get() + 1);
    }

    pub fn remove(&self, key: u32) -> Option<Entry> {
        return self.store.borrow_mut().remove(&key);
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
