//! Dialogue Exporter - Exportar diálogos a CSV, JSON y Rust

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DialogueLine {
    pub id: String,
    pub text: String,
    pub speaker: Option<String>,
    pub action: String,
    pub duration_ms: u32,
    pub is_hidden: bool,
    pub variables: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Dialogue {
    pub id: String,
    pub lines: Vec<DialogueLine>,
    pub language: String,
    pub variables: HashMap<String, String>,
}

pub struct DialogueExporter;

impl DialogueExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export_to_csv(dialogue: &Dialogue) -> Result<String, Box<dyn std::error::Error>> {
        let mut csv = String::new();
        csv.push_str("id,text,speaker,action,duration_ms,is_hidden,variables\n");
        for line in &dialogue.lines {
            let escaped_text = line.text.replace("\\", "\\\\").replace("\"", "\"\"");
            let escaped_speaker = line.speaker.as_deref().map(|s| s.replace("\\", "\\\\")).unwrap_or_default();
            let escaped_vars = line.variables.join(",");
            csv.push_str(&format!("\"{}\",\"{}\",\"{}\",{:?},{},{},\"\n",
                line.id,
                escaped_text,
                escaped_speaker,
                line.action,
                line.duration_ms,
                line.is_hidden,
            ));
        }
        Ok(csv)
    }
    
    pub fn export_to_json(dialogue: &Dialogue) -> String {
        format!("{{\"id\":\"{}\",\"language\":\"{}\",\"lines\":[]}}",
            dialogue.id,
            dialogue.language,
        )
    }
    
    pub fn export_to_rust(dialogue: &Dialogue) -> String {
        format!("pub fn dialogue_{}() {{}}", dialogue.id)
    }
}
