#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ImportQueue {
    pub queue: Vec<String>,
}

#[allow(dead_code)]
impl ImportQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn push(&mut self, path: String) {
        self.queue.push(path);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.queue.pop()
    }
}

impl Default for ImportQueue {
    fn default() -> Self {
        Self::new()
    }
}