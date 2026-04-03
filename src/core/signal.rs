// Import //

use std::sync::{Arc, Mutex};

// Types //

type Callback<T> = Box<dyn Fn(T) + Send + Sync>;

// Class //

pub struct Event<T: Clone> {
    listeners: Arc<Mutex<Vec<Callback<T>>>>
}

impl <T: Clone + Send + 'static> Event<T> {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(vec![]))
        }
    }
    
    pub fn connect(&self, callback: Callback<T>) {
        self.listeners.lock().unwrap().push(callback);
    }
    
    pub fn fire(&self, data: T) {
        for callback in self.listeners.lock().unwrap().iter() {
            callback(data.clone());
        }
    }
}