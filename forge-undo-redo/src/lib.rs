use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoCommand {
    pub command: String,
    pub data: serde_json::Value,
}

pub struct UndoStack {
    pub history: Vec<UndoCommand>,
    pub max_size: usize,
}

impl UndoStack {
    pub fn new(max_size: usize) -> Self {
        Self {
            history: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, command: UndoCommand) {
        self.history.push(command);
        if self.history.len() > self.max_size {
            self.history.remove(0);
        }
    }

    pub fn undo(&mut self) -> Option<UndoCommand> {
        self.history.pop()
    }

    pub fn redo(&mut self) -> Option<UndoCommand> {
        Some(self.history.last().cloned()?)
    }
}

pub struct UndoManager {
    pub stack: UndoStack,
}

impl UndoManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            stack: UndoStack::new(max_size),
        }
    }

    pub fn undo(&mut self) {
        self.stack.undo();
    }

    pub fn redo(&mut self) {
        self.stack.redo();
    }
}
