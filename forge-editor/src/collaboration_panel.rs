//! # Collaboration Panel API
//! 
//! Módulo para panel de colaboración.

use eframe::egui;
use crate::collaboration::{UserInfo, ChatMessage};

/// Collaboration Panel - panel para colaboración
#[derive(Default)]
pub struct CollaborationPanel {
    pub users: Vec<UserInfo>,
    pub messages: Vec<ChatMessage>,
}

impl CollaborationPanel {
    pub fn new() -> Self {
        Self::default()
    }
}

