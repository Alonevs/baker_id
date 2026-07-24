//! Sistema de diálogos

use serde::{Deserialize, Serialize};

/// Tipo de diálogo
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DialogueType {
    Story,
    Interactive,
    System,
}

impl Default for DialogueType {
    fn default() -> Self {
        DialogueType::Story
    }
}

/// Línea de diálogo
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DialogueLine {
    pub text: String,
    pub speaker: String,
    pub action: String,
    pub choices: Vec<DialogueChoice>,
}

impl Default for DialogueLine {
    fn default() -> Self {
        Self {
            text: String::new(),
            speaker: String::new(),
            action: String::new(),
            choices: Vec::new(),
        }
    }
}

/// Opción de diálogo
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DialogueChoice {
    pub text: String,
    pub next_line_index: usize,
    pub trigger_event: Option<String>,
}

impl Default for DialogueChoice {
    fn default() -> Self {
        Self {
            text: String::new(),
            next_line_index: 0,
            trigger_event: None,
        }
    }
}

/// Diálogo completo
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Dialogue {
    pub id: u64,
    pub title: String,
    pub type_: DialogueType,
    pub lines: Vec<DialogueLine>,
    pub current_line: usize,
    pub is_active: bool,
}

impl Dialogue {
    pub fn new(id: u64, title: String) -> Self {
        Self {
            id,
            title,
            type_: DialogueType::default(),
            lines: Vec::new(),
            current_line: 0,
            is_active: false,
        }
    }

    pub fn add_line(&mut self, text: String, speaker: String) {
        let line = DialogueLine {
            text,
            speaker,
            action: String::new(),
            choices: Vec::new(),
        };
        self.lines.push(line);
    }

    pub fn get_current_line(&self) -> Option<&DialogueLine> {
        if self.current_line < self.lines.len() {
            Some(&self.lines[self.current_line])
        } else {
            None
        }
    }

    pub fn next_line(&mut self) {
        if self.current_line + 1 < self.lines.len() {
            self.current_line += 1;
        }
    }

    pub fn choose(&mut self, choice_index: usize) {
        if let Some(line) = self.get_current_line() {
            if choice_index < line.choices.len() {
                self.current_line = line.choices[choice_index].next_line_index;
            }
        }
    }

    pub fn end(&mut self) {
        self.is_active = false;
        self.current_line = 0;
    }
}

/// Sistema de diálogos
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DialogueSystem {
    pub dialogues: Vec<Dialogue>,
    pub active_dialogue: Option<u64>,
}

impl DialogueSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_dialogue(&mut self, id: u64, title: String) -> &mut Dialogue {
        let dialogue = Dialogue::new(id, title);
        self.dialogues.push(dialogue);
        self.dialogues.last_mut().unwrap()
    }

    pub fn get_dialogue(&self, id: u64) -> Option<&Dialogue> {
        self.dialogues.iter().find(|d| d.id == id)
    }

    pub fn get_dialogue_mut(&mut self, id: u64) -> Option<&mut Dialogue> {
        self.dialogues.iter_mut().find(|d| d.id == id)
    }

    pub fn activate_dialogue(&mut self, id: u64) -> Option<&mut Dialogue> {
        if let Some(dialogue) = self.get_dialogue_mut(id) {
            dialogue.is_active = true;
            dialogue.current_line = 0;
            Some(dialogue)
        } else {
            None
        }
    }
}
