use std::collections::HashMap;
use crate::dialogue_system::{DialogueSystem, Dialogue, DialogueOption, DialogueAction};
use crate::dialogue_ui::dialogue_panel::DialogueUI;

pub struct DialogueManager {
    dialogue_system: DialogueSystem,
    current_state: DialogueState,
}

enum DialogueState {
    Idle,
    Active { dialogue_id: String },
}

impl DialogueManager {
    pub fn new() -> Self {
        DialogueManager {
            dialogue_system: DialogueSystem::new(),
            current_state: DialogueState::Idle,
        }
    }

    pub fn add_dialogue(&mut self, dialogue: Dialogue) {
        self.dialogue_system.add_dialogue(dialogue);
    }

    pub fn start_dialogue(&mut self, dialogue_id: &str) {
        self.current_state = DialogueState::Active {
            dialogue_id: dialogue_id.to_string(),
        };
    }

    pub fn handle_dialogue_input(&mut self, key: KeyCode) {
        match &mut self.current_state {
            DialogueState::Active { dialogue_id } => {
                if let Some(dialogue) = self.dialogue_system.get_current_dialogue() {
                    match key {
                        KeyCode::Enter => {
                            // Next line
                        }
                        KeyCode::Down | KeyCode::KeyN => {
                            // Next option
                            if let Some(option) = dialogue.select_option("next") {
                                self.execute_dialogue_action(&option.action);
                            }
                        }
                        _ => {}
                    }
                }
            }
            DialogueState::Idle => {}
        }
    }

    fn execute_dialogue_action(&mut self, action: &DialogueAction) {
        match action {
            DialogueAction::NextDialogue(next_id) => {
                self.start_dialogue(next_id);
            }
            DialogueAction::SetVariable(key, value) => {
                self.dialogue_system.set_variable(key, *value);
            }
            DialogueAction::TriggerEvent(event_id) => {
                // Trigger game event
                println!("Triggering event: {}", event_id);
            }
        }
    }

    pub fn check_variable(&self, key: &str) -> bool {
        self.dialogue_system.get_variable(key).unwrap_or(false)
    }

    pub fn set_variable(&mut self, key: &str, value: bool) {
        self.dialogue_system.set_variable(key, value);
    }
}

enum DialogueState {
    Idle,
    Active { dialogue_id: String },
}
