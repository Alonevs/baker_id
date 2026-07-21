use crate::dialogue_editor::{Dialogue, DialogueLine, DialogueAction, VariableType, ConditionOperator, DialogueVariable, Actor, DialogueCondition};
use std::fs;

pub struct DialogueExporter;

impl DialogueExporter {
    /// Exportar diálogo a formato JSON
    pub fn export_to_json(dialogue: &Dialogue) -> String {
        serde_json::to_string_pretty(dialogue).unwrap_or_else(|_| "Error serializing to JSON".to_string())
    }
    
    /// Exportar múltiples diálogos a archivo JSON
    pub fn export_dialogues_to_file(dialogues: &[(String, Dialogue)], path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut all_dialogues = Vec::new();
        for (id, dialogue) in dialogues {
            all_dialogues.push((id.clone(), serde_json::to_value(dialogue)?));
        }
        
        let json = serde_json::to_string_pretty(&all_dialogues)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    /// Exportar diálogo a código Rust
    pub fn export_to_rust(dialogue: &Dialogue) -> String {
        let mut code = String::new();
        code.push_str(&format!(
            "// Dialogue: {}\n",
            dialogue.name
        ));
        code.push_str(&format!(
            "// Actor: {:?}",
            dialogue.actor_id
        ));
        code.push_str(&format!(
            "// Language: {}\n",
            dialogue.language
        ));
        code.push_str("\n");
        
        // Exportar líneas
        code.push_str("pub fn dialogue_lines() -> Vec<DialogueLine> {\n");
        for line in &dialogue.lines {
            code.push_str(&format!(
                "    DialogueLine {{\n\
                id: \"{}\",\n\
                text: \"{}\",\n\
                speaker: Some(\"{}\".to_string()),\n\
                action: DialogueAction::Speak,\n\
                duration_ms: {},\n\
                is_hidden: false,\n\
                variables: vec![],\n\
            }},\n",
                line.id,
                escape_rust_string(&line.text),
                line.speaker.as_deref().unwrap_or(""),
                line.duration_ms
            ));
        }
        code.push_str("}\n\n");
        
        // Exportar condiciones
        if !dialogue.conditions.is_empty() {
            code.push_str("pub fn dialogue_conditions() -> Vec<DialogueCondition> {\n");
            for condition in &dialogue.conditions {
                code.push_str(&format!(
                    "    DialogueCondition {{\n\
                    id: \"{}\",\n\
                    operator: {:?},\n\
                    left_value: serde_json::json!({}),\n\
                    right_value: serde_json::json!({}),\n\
                    is_negated: {},\n\
                }},\n",
                    condition.id,
                    condition.operator,
                    condition.left_value,
                    condition.right_value,
                    condition.is_negated
                ));
            }
            code.push_str("}\n\n");
        }
        
        // Exportar variables
        if !dialogue.variables.is_empty() {
            code.push_str("pub fn dialogue_variables() -> HashMap<String, DialogueVariable> {\n");
            for (var_id, var) in &dialogue.variables {
                code.push_str(&format!(
                    "    \"{}\" => DialogueVariable {{\n\
                    id: \"{}\",\n\
                    name: \"{}\",\n\
                    variable_type: {:?},\n\
                    value: serde_json::json!({:?}),\n\
                    is_global: {},\n\
                    description: \"{}\",\n\
                }},\n",
                    var_id,
                    var.id,
                    var.name,
                    var.variable_type,
                    var.value,
                    var.is_global,
                    var.description
                ));
            }
            code.push_str("}\n\n");
        }
        
        code
    }
    
    /// Exportar diálogo a CSV
    pub fn export_to_csv(dialogue: &Dialogue) -> String {
        let mut csv = String::new();
        csv.push_str(&format!("id,text,speaker,action,duration_ms,is_hidden,variables\n"));
        
        for line in &dialogue.lines {
            let escaped_text = csv_escape(&line.text);
            let escaped_speaker = line.speaker.as_deref().map(csv_escape).unwrap_or_default();
            let escaped_vars = line.variables.join(",");
            
            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",{:?},{},{},\"\n",
                line.id,
                escaped_text,
                escaped_speaker,
                line.action,
                line.duration_ms,
                line.is_hidden,
                escaped_vars
            ));
        }
        
        csv
    }
    
    /// Exportar todos los diálogos a CSV
    pub fn export_all_dialogues_to_csv(dialogues: &[(String, Dialogue)], path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_content = String::new();
        csv_content.push_str(&format!("dialogue_id,line_id,text,speaker,action,duration_ms,is_hidden,variables\n"));
        
        for (dialogue_id, dialogue) in dialogues {
            for line in &dialogue.lines {
                let escaped_text = csv_escape(&line.text);
                let escaped_speaker = line.speaker.as_deref().map(csv_escape).unwrap_or_default();
                let escaped_vars = line.variables.join(",");
                
                csv_content.push_str(&format!(
                    "\"{}\",\"{}\",\"{}\",{:?},{},{},\"\n",
                    dialogue_id,
                    line.id,
                    escaped_text,
                    escaped_speaker,
                    line.action,
                    line.duration_ms,
                    line.is_hidden,
                    escaped_vars
                ));
            }
        }
        
        fs::write(path, csv_content)?;
        Ok(())
    }
}

/// Escapar string para Rust
fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

/// Escapar string para CSV
fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialogue_editor::{Dialogue, DialogueLine, DialogueAction};

    #[test]
    fn test_export_to_json() {
        let dialogue = Dialogue {
            id: "test_dialog".to_string(),
            name: "Test Dialogue".to_string(),
            actor_id: Some("hero".to_string()),
            language: "es".to_string(),
            lines: vec![
                DialogueLine {
                    id: "line1".to_string(),
                    text: "Hello".to_string(),
                    speaker: Some("hero".to_string()),
                    action: DialogueAction::Speak,
                    duration_ms: 2000,
                    is_hidden: false,
                    variables: vec![],
                },
            ],
            conditions: vec![],
            variables: HashMap::new(),
            is_active: true,
            priority: 0,
        };
        
        let json = DialogueExporter::export_to_json(&dialogue);
        assert!(!json.is_empty());
    }
    
    #[test]
    fn test_export_to_csv() {
        let dialogue = Dialogue {
            id: "test_dialog".to_string(),
            name: "Test Dialogue".to_string(),
            actor_id: Some("hero".to_string()),
            language: "es".to_string(),
            lines: vec![
                DialogueLine {
                    id: "line1".to_string(),
                    text: "Hello, World!".to_string(),
                    speaker: Some("hero".to_string()),
                    action: DialogueAction::Speak,
                    duration_ms: 2000,
                    is_hidden: false,
                    variables: vec![],
                },
            ],
            conditions: vec![],
            variables: HashMap::new(),
            is_active: true,
            priority: 0,
        };
        
        let csv = DialogueExporter::export_to_csv(&dialogue);
        assert!(csv.contains("Hello, World!"));
    }
}

