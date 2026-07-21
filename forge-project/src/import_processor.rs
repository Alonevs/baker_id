use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ImportProcessor {
    pub processed: HashMap<String, bool>,
}

#[allow(dead_code)]
impl ImportProcessor {
    pub fn new() -> Self {
        Self { processed: HashMap::new() }
    }

    pub fn process(&mut self, path: &str) -> Result<(), String> {
        self.processed.insert(path.to_string(), true);
        Ok(())
    }

    pub fn is_processed(&self, path: &str) -> bool {
        self.processed.get(path).map(|&b| b).unwrap_or(false)
    }
}

impl Default for ImportProcessor {
    fn default() -> Self {
        Self::new()
    }
}