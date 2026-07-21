use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FormatConverter {
    pub formats: HashMap<String, String>,
}

#[allow(dead_code)]
impl FormatConverter {
    pub fn new() -> Self {
        Self { formats: HashMap::new() }
    }

    pub fn convert(&self, from: &str, to: &str) -> bool {
        self.formats.contains_key(from) && self.formats.contains_key(to)
    }
}

impl Default for FormatConverter {
    fn default() -> Self {
        Self::new()
    }
}