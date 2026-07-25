use std::collections::HashMap;

pub struct Dialogue {
    id: String,
    speaker: String,
    text: String,
    options: Vec<DialogueOption>,
}

impl Dialogue {
    pub fn new(id: &str, speaker: &str, text: &str, options: Vec<DialogueOption>) -> Self {
        Dialogue {
            id: id.to_string(),
            speaker: speaker.to_string(),
            text: text.to_string(),
            options,
        }
    }

    pub fn show(&self) {
        println!("{}: {}", self.speaker, self.text);
        for option in &self.options {
            println!("  [{}] {}", option.id, option.text);
        }
    }

    pub fn select_option(&mut self, option_id: &str) -> Option<&DialogueOption> {
        self.options.iter().find(|o| o.id == option_id)
    }
}

pub struct DialogueOption {
    pub id: String,
    pub text: String,
    pub action: DialogueAction,
}

pub enum DialogueAction {
    NextDialogue(String),
    SetVariable(String, bool),
    TriggerEvent(String),
}

pub struct DialogueSystem {
    dialogues: HashMap<String, Dialogue>,
    current_dialogue: Option<String>,
    variables: HashMap<String, bool>,
}

impl DialogueSystem {
    pub fn new() -> Self {
        DialogueSystem {
            dialogues: HashMap::new(),
            current_dialogue: None,
            variables: HashMap::new(),
        }
    }

    pub fn add_dialogue(&mut self, dialogue: Dialogue) {
        self.dialogues.insert(dialogue.id, dialogue);
    }

    pub fn start_dialogue(&mut self, dialogue_id: &str) -> Option<Dialogue> {
        self.current_dialogue = Some(dialogue_id.to_string());
        self.dialogues.get(dialogue_id).cloned()
    }

    pub fn get_current_dialogue(&self) -> Option<&Dialogue> {
        self.current_dialogue.as_ref().and_then(|id| self.dialogues.get(id))
    }

    pub fn set_variable(&mut self, key: &str, value: bool) {
        self.variables.insert(key.to_string(), value);
    }

    pub fn get_variable(&self, key: &str) -> Option<bool> {
        self.variables.get(key).cloned()
    }

    pub fn has_variable(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }
}
