use parking_lot::RwLock;
use std::sync::Arc;

pub struct SharedState {
    pub data: Arc<RwLock<Vec<u8>>>,
    pub listeners: Vec<Box<dyn Fn()>>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
            listeners: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        // Inicialización
    }

    pub fn update(&self) {
        for listener in &self.listeners {
            listener();
        }
    }
}

pub struct SyncSystem;

impl SyncSystem {
    pub fn sync_state(&self, state: &SharedState) {
        // Sincronización
    }
}
