use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: String,
    pub documentation: Option<String>,
    pub text_edit: Option<TextEdit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionKind {
    Keyword,
    Function,
    Method,
    Class,
    Struct,
    Module,
    Variable,
    Property,
    Field,
    Constructor,
    Enum,
    EnumMember,
    Event,
    Operator,
    TypeParameter,
    Unit,
    Value,
    Color,
    File,
    Reference,
    Folder,
    Customcolor,
    Constant,
    Snippet,
    ColorPicker,
    FileFolder,
    Macro,
    ReferenceModifier,
    Parameter,
    Type,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

pub struct CompletionProvider {
    pub completions: HashMap<String, Vec<CompletionItem>>,
}

impl CompletionProvider {
    pub fn new() -> Self {
        Self {
            completions: HashMap::new(),
        }
    }

    pub fn get_completions(&self, prefix: &str) -> Vec<&CompletionItem> {
        self.completions
            .get(prefix)
            .map(|items| items.iter())
            .unwrap_or_default()
            .collect()
    }

    pub fn add_keyword(&mut self, keyword: &str, item: CompletionItem) {
        self.completions
            .entry(keyword.to_string())
            .or_default()
            .push(item);
    }
}

impl Default for CompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}
