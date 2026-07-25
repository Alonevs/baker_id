use crate::dialogue_system::{Dialogue, DialogueSystem};

pub struct DialogueUI {
    dialogue_system: DialogueSystem,
    is_visible: bool,
    current_dialogue: Option<Dialogue>,
    text_position: Vec2,
}

impl DialogueUI {
    pub fn new(dialogue_system: DialogueSystem) -> Self {
        DialogueUI {
            dialogue_system,
            is_visible: false,
            current_dialogue: None,
            text_position: Vec2::new(100.0, 100.0),
        }
    }

    pub fn show_dialogue(&mut self, dialogue_id: &str) {
        if let Some(dialogue) = self.dialogue_system.start_dialogue(dialogue_id) {
            self.current_dialogue = Some(dialogue);
            self.is_visible = true;
        }
    }

    pub fn hide_dialogue(&mut self) {
        self.is_visible = false;
        self.current_dialogue = None;
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        if let Some(ref mut dialogue) = self.current_dialogue {
            match key {
                KeyCode::Enter => {
                    // Next line or option
                }
                KeyCode::Down | KeyCode::KeyN => {
                    // Select next option
                }
                _ => {}
            }
        }
    }

    pub fn draw(&self) {
        if self.is_visible {
            if let Some(ref dialogue) = self.current_dialogue {
                // Draw dialogue panel
                println!("=== DIALOGUE ===");
                println!("{}: {}", dialogue.speaker, dialogue.text);
                for option in &dialogue.options {
                    println!("  [{}] {}", option.id, option.text);
                }
                println!("================");
            }
        }
    }
}
