use std::collections::HashMap;
use std::fs;

pub struct BitacoraValidator {
    pub excel_references: HashMap<String, String>,
    pub bitacora_path: String,
}

impl Default for BitacoraValidator {
    fn default() -> Self {
        Self {
            excel_references: HashMap::new(),
            bitacora_path: "bitacora.md".to_string(),
        }
    }
}

impl BitacoraValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_excel_references(&mut self, references: &HashMap<String, String>) {
        self.excel_references = references.clone();
    }

    pub fn validate_bitacora(&mut self) {
        let content = match fs::read_to_string(&self.bitacora_path) {
            Ok(c) => c,
            Err(_) => return,
        };

        // Validación básica
        if content.is_empty() {
            println!("⚠️ Bitacora vacía");
        }
    }

    pub fn get_errors(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn get_warnings(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn get_infos(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn has_errors(&self) -> bool {
        false
    }

    pub fn print_all(&self) {
        // No hacer nada
    }
}

pub fn validate_and_get_results(validator: &BitacoraValidator) -> (Vec<String>, Vec<String>, Vec<String>) {
    let errors: Vec<String> = validator.get_errors().clone();
    let warnings: Vec<String> = validator.get_warnings().clone();
    let infos: Vec<String> = validator.get_infos().clone();
    (errors, warnings, infos)
}

