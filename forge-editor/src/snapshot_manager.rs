//! # Snapshot Manager
//! 
//! Gestiona snapshots de escenas para Play Mode:
//! - Serialización/deserialización
//! - Restauración de estado
//! - Persistencia temporal

use crate::play_session::{Entity, SceneSnapshot};

/// Manager de snapshots para Play Mode
#[derive(Debug)]
pub struct SnapshotManager {
    /// Snapshot actual de la escena
    pub current_snapshot: SceneSnapshot,
    /// Historial de snapshots (últimos 10)
    pub history: Vec<SceneSnapshot>,
}

impl SnapshotManager {
    /// Crea un nuevo snapshot manager
    pub fn new() -> Self {
        Self {
            current_snapshot: SceneSnapshot::new(),
            history: Vec::new(),
        }
    }

    /// Toma un snapshot de la escena actual
    pub fn take_snapshot(&mut self, entities: &[Entity]) {
        let snapshot = SceneSnapshot::from_entities(entities);
        
        // Agregar al historial (máximo 10)
        self.history.push(snapshot.clone());
        if self.history.len() > 10 {
            self.history.remove(0);
        }
        
        self.current_snapshot = snapshot;
    }

    /// Restaura la escena desde el snapshot actual
    pub fn restore_snapshot(&mut self, entities: &mut [Entity]) {
        self.current_snapshot.restore_to_entities(entities);
    }

    /// Restaura desde un snapshot específico del historial
    pub fn restore_from_history(&mut self, index: usize, entities: &mut [Entity]) {
        if index < self.history.len() {
            let snapshot = self.history[index].clone();
            snapshot.restore_to_entities(entities);
        }
    }

    /// Obtiene el snapshot más reciente del historial
    pub fn get_last_snapshot(&self) -> Option<&SceneSnapshot> {
        self.history.last()
    }

    /// Obtiene el snapshot más reciente mutado
    pub fn get_last_snapshot_mut(&mut self) -> Option<&mut SceneSnapshot> {
        self.history.last_mut()
    }

    /// Elimina el snapshot actual del historial
    pub fn clear_current(&mut self) {
        self.current_snapshot = SceneSnapshot::new();
        self.history.retain(|s| s.timestamp != self.current_snapshot.timestamp);
    }

    /// Obtiene el número de snapshots en el historial
    pub fn get_history_count(&self) -> usize {
        self.history.len()
    }

    /// Obtiene el snapshot actual
    pub fn get_current(&self) -> &SceneSnapshot {
        &self.current_snapshot
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}
