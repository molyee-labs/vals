use std::sync::{Mutex};

pub struct Holder<'a, T: Default> {
    pool: &'a Pool<T>,
    entry: T,
}

pub struct Pool<T: Default> {
    items: Mutex<Vec<T>>
}

impl<T> Pool<T> {
    fn new() -> Self {
        Pool { items: Mutex::new(vec![]) }
    }

    fn take(&self) -> Holder<T> {
        let _guard = self.items.lock().unwrap();
        let entry = self.items.pop().unwrap_or_default();
        Holder { pool: &self, entry }
    }

    fn release(&self, entry: T) {
        let _guard = self.items.lock().unwrap();
        self.items.push(entry);
    }
}


/*pub struct Pool<T: Default + Ord> {
    gate: Mutex<i32>,
    open: Vec<T>,
    busy: Vec<T>,
}

pub struct Holder<'a, 'b: 'a, T: 'b + Default + Ord> {
    pool: &'a Pool<T>,
    owned: &'b T,
}

impl<'a, 'b, T: 'b + Default + Ord> Holder<'a, 'b, T> {
    fn get(&self) -> &T {
        self.owned
    }
}

impl<'a, 'b, T: 'b + Default + Ord> Drop for Holder<'a, 'b, T> {
    fn drop(&mut self) {
        self.pool.release(&self.owned)
    }
}

impl<T: Default + Ord> Pool<T> {
    pub fn new() -> Self {
        Pool {
            gate: Mutex::new(0),
            open: vec![],
            busy: vec![],
        }
    }

    pub fn take(&mut self) -> Holder<T> {
        // TODO no unwrap but recreate mutex if it's poisoned
        let _guard = self.gate.lock().unwrap();
        let entry = self.open.pop().unwrap_or_default();
        self.busy.push(entry);
        Holder { pool: &self, owned: &entry }
    }

    pub fn release(&mut self, entry: &T) {
        let _guard = self.gate.lock().unwrap();
        let index = self.busy.binary_search(entry)?;
        self.busy.swap_remove(index);
        self.open.push(&entry);
    }
}*/
