use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Tipos de mensajes del linter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinterMessage {
    Info { message: String },
    Warning { message: String },
    Error { message: String },
}

impl std::fmt::Display for LinterMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinterMessage::Info { message } => write!(f, "[INFO] {}", message),
            LinterMessage::Warning { message } => write!(f, "[WARNING] {}", message),
            LinterMessage::Error { message } => write!(f, "[ERROR] {}", message),
        }
    }
}

/// Gestor del sistema de linter para la bitácora
pub struct BitacoraLinter {
    pub messages: Vec<LinterMessage>,
    pub valid_references: HashMap<String, String>,
}

impl Default for BitacoraLinter {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            valid_references: HashMap::new(),
        }
    }
}

impl BitacoraLinter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_references(&mut self, references: &HashMap<String, String>) {
        self.valid_references = references.clone();
    }

    /// Analizar bitácora markdown
    pub fn analyze_bitacora(&mut self, content: &str, bitacora_path: &str) {
        self.messages.clear();
        
        let re = Regex::new(r"\{(\w+):([^}]+)\}").unwrap();
        
        for cap in re.captures_iter(content) {
            let ty = cap.get(1).unwrap().as_str();
            let id = cap.get(2).unwrap().as_str();
            
            match ty {
                "note" => {
                    let note_text = id;
                    self.messages.push(LinterMessage::Info {
                        message: note_text.to_string(),
                    });
                }
                "actor" | "dlg" | "evt" | "var" | "scene" => {
                    let full_id = format!("{}_{}", ty, id);
                    if !self.valid_references.contains_key(&full_id) {
                        self.messages.push(LinterMessage::Error {
                            message: format!(
                                "Referencia rota en {}: ID '{}' no existe en el Excel",
                                bitacora_path, full_id
                            ),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    /// Detectar IDs duplicados en Excel
    pub fn check_duplicates(&mut self, references: &HashMap<String, String>) {
        let mut counts: HashMap<String, u32> = HashMap::new();
        
        for id in references.keys() {
            *counts.entry(id.clone()).or_insert(0) += 1;
        }
        
        for (id, count) in &counts {
            if *count > 1 {
                self.messages.push(LinterMessage::Error {
                    message: format!("ID duplicado en Excel: '{}'", id),
                });
            }
        }
    }

    pub fn get_errors(&self) -> Vec<&LinterMessage> {
        self.messages.iter().filter(|m| matches!(m, LinterMessage::Error { .. })).collect()
    }

    pub fn get_warnings(&self) -> Vec<&LinterMessage> {
        self.messages.iter().filter(|m| matches!(m, LinterMessage::Warning { .. })).collect()
    }

    pub fn get_infos(&self) -> Vec<&LinterMessage> {
        self.messages.iter().filter(|m| matches!(m, LinterMessage::Info { .. })).collect()
    }

    pub fn has_errors(&self) -> bool {
        !self.get_errors().is_empty()
    }

    pub fn print_messages(&self) {
        for msg in &self.messages {
            println!("{}", msg);
        }
    }
}

/// Parseador de patrones de la bitácora
pub fn parse_bitacora_pattern(text: &str) -> Option<(String, String)> {
    let re = Regex::new(r"\{(\w+):([^}]+)\}").unwrap();
    let mut caps = re.captures_iter(text);
    
    if let Some(cap) = caps.next() {
        let ty = cap.get(1).unwrap().as_str();
        let id = cap.get(2).unwrap().as_str();
        Some((ty.to_string(), id.to_string()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bitacora_pattern() {
        let result = parse_bitacora_pattern("{evt:evt_test}");
        assert_eq!(result, Some(("evt".to_string(), "evt_test".to_string())));
    }

    #[test]
    fn test_linter_detects_missing_reference() {
        let mut linter = BitacoraLinter::new();
        let valid_refs = HashMap::new();
        linter.load_references(&valid_refs);
        
        let content = "{evt:evt_missing}";
        linter.analyze_bitacora(content, "bitacora.md");
        
        assert!(linter.get_errors().len() == 1);
    }
}
