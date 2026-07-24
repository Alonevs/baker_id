//! Live Sync Manager - Sincronización en tiempo real

use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SyncEvent {
    pub entity_id: u64,
    pub property: String,
    pub old_value: String,
    pub new_value: String,
}

impl Default for SyncEvent {
    fn default() -> Self {
        Self {
            entity_id: 0,
            property: String::new(),
            old_value: String::new(),
            new_value: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiveSyncManager {
    pub subscribers: HashMap<u64, Vec<SyncEvent>>,
    pub pending_events: Vec<SyncEvent>,
}

impl Default for LiveSyncManager {
    fn default() -> Self {
        Self {
            subscribers: HashMap::new(),
            pending_events: Vec::new(),
        }
    }
}

impl LiveSyncManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn subscribe(&mut self, entity_id: u64) {
        self.subscribers.entry(entity_id).or_default();
    }
    
    pub fn get_subscribers(&self, entity_id: u64) -> Vec<&Vec<SyncEvent>> {
        self.subscribers.get(&entity_id).map(|v| vec![v]).unwrap_or_default()
    }
    
    pub fn add_event(&mut self, event: SyncEvent) {
        self.pending_events.push(event);
    }
    
    pub fn get_pending_events(&self) -> Vec<&SyncEvent> {
        self.pending_events.iter().collect()
    }
    
    pub fn clear_events(&mut self) {
        self.pending_events.clear();
    }
    
    pub fn show_sync_panel(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("sync_panel").show(ctx, |ui| {
            ui.label(egui::RichText::new("Live Sync").color(egui::Color32::WHITE));
            ui.label(format!("Pending: {}", self.pending_events.len()));
        });
    }
}
