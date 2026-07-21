//! forge-undo-redo - Sistema de deshacer/rehacer

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoState {
    pub action: String,
    pub state_snapshot: serde_json::Value,
    pub timestamp: u64,
}

pub struct UndoStack {
    pub undo_stack: VecDeque<UndoState>,
    pub redo_stack: VecDeque<UndoState>,
    pub max_history: usize,
}

impl UndoStack {
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(max_history),
            redo_stack: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    pub fn push(&mut self, action: String, state_snapshot: serde_json::Value) {
        self.redo_stack.clear();
        
        if self.undo_stack.len() >= self.max_history {
            self.undo_stack.pop_front();
        }
        
        self.undo_stack.push_back(UndoState {
            action,
            state_snapshot,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        });
    }

    pub fn undo(&mut self) -> Option<UndoState> {
        let state = self.undo_stack.pop_back()?;
        self.redo_stack.push_back(state.clone());
        Some(state)
    }

    pub fn redo(&mut self) -> Option<UndoState> {
        let state = self.redo_stack.pop_back()?;
        self.undo_stack.push_back(state.clone());
        Some(state)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

pub type StateSnapshot = serde_json::Value;

pub struct UndoManager {
    pub stack: Arc<Mutex<UndoStack>>,
}

impl UndoManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            stack: Arc::new(Mutex::new(UndoStack::new(max_history))),
        }
    }

    pub fn record(&self, action: String, snapshot: StateSnapshot) {
        let mut stack = self.stack.lock().unwrap();
        stack.push(action, snapshot);
    }

    pub fn undo(&self) -> Option<UndoState> {
        let mut stack = self.stack.lock().unwrap();
        stack.undo()
    }

    pub fn redo(&self) -> Option<UndoState> {
        let mut stack = self.stack.lock().unwrap();
        stack.redo()
    }

    pub fn can_undo(&self) -> bool {
        let stack = self.stack.lock().unwrap();
        stack.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        let stack = self.stack.lock().unwrap();
        stack.can_redo()
    }
}

impl Default for UndoManager {
    fn default() -> Self {
        Self::new(100)
    }
}
