use std::collections::HashMap;
use std::fs;
use forge_linter::{BitacoraLinter, LinterMessage};

pub struct BitacoraValidator {
    pub linter: BitacoraLinter,
    pub excel_references: HashMap<String, String>,
    pub bitacora_path: String,
}

impl Default for BitacoraValidator {
    fn default() -> Self {
        Self {
            linter: BitacoraLinter::new(),
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
        self.linter.load_references(references);
    }

    pub fn validate_bitacora(&mut self) {
        let content = match fs::read_to_string(&self.bitacora_path) {
            Ok(c) => c,
            Err(_) => return,
        };

        self.linter.analyze_bitacora(&content, &self.bitacora_path);
        self.linter.check_duplicates(&self.excel_references);
    }

    pub fn get_errors(&self) -> Vec<&LinterMessage> {
        self.linter.get_errors()
    }

    pub fn get_warnings(&self) -> Vec<&LinterMessage> {
        self.linter.get_warnings()
    }

    pub fn get_infos(&self) -> Vec<&LinterMessage> {
        self.linter.get_infos()
    }

    pub fn has_errors(&self) -> bool {
        self.linter.has_errors()
    }

    pub fn print_all(&self) {
        self.linter.print_messages();
    }
}

pub fn validate_and_get_results(validator: &BitacoraValidator) -> (Vec<String>, Vec<String>, Vec<String>) {
    let errors: Vec<String> = validator.get_errors().iter().map(|m| m.to_string()).collect();
    let warnings: Vec<String> = validator.get_warnings().iter().map(|m| m.to_string()).collect();
    let infos: Vec<String> = validator.get_infos().iter().map(|m| m.to_string()).collect();
    (errors, warnings, infos)
}

