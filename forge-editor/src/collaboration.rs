use std::collections::HashMap;
use crate::hot_reload::ChangeType;

/// Selection range
#[derive(Debug, Clone)]
pub struct SelectionRange {
    pub start: usize,
    pub end: usize,
}

impl SelectionRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// User info
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub status: UserStatus,
    pub is_owner: bool,
    pub last_seen: u64,
    pub joined_at: u64,
    pub avatar: Option<String>,
}

impl UserInfo {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            status: UserStatus::Offline,
            is_owner: false,
            last_seen: 0,
            joined_at: 0,
            avatar: None,
        }
    }
    
    pub fn set_owner(&mut self) {
        self.is_owner = true;
    }
    
    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }
}

/// User status
#[derive(Debug, Clone, PartialEq)]
pub enum UserStatus {
    Online,
    Offline,
    Away,
    Busy,
}

/// Collaboration session
#[derive(Debug, Clone)]
pub struct CollaborationSession {
    pub session_id: String,
    pub project_id: String,
    pub host_id: String,
    pub created_at: u64,
    pub users: HashMap<String, UserInfo>,
    pub chat_history: Vec<ChatMessage>,
    pub clipboard: Option<SharedClipboard>,
    pub cursors: HashMap<String, UserCursor>,
}

impl CollaborationSession {
    pub fn new(session_id: String, project_id: String, host_id: String) -> Self {
        Self {
            session_id,
            project_id,
            host_id,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            users: HashMap::new(),
            chat_history: Vec::new(),
            clipboard: None,
            cursors: HashMap::new(),
        }
    }

    pub fn get_cursors(&self) -> &HashMap<String, UserCursor> {
        &self.cursors
    }
    
    pub fn add_user(&mut self, user_id: String, user_info: UserInfo) {
        self.users.insert(user_id, user_info);
    }

    pub fn remove_user(&mut self, user_id: &str) -> Option<UserInfo> {
        self.users.remove(user_id)
    }

    pub fn add_cursor(&mut self, cursor: UserCursor) {
        self.cursors.insert(cursor.user_id.clone(), cursor);
    }
    
    pub fn get_user(&self, user_id: &str) -> Option<&UserInfo> {
        self.users.get(user_id)
    }
    
    pub fn user_count(&self) -> usize {
        self.users.len()
    }
    
    pub fn add_message(&mut self, msg: ChatMessage) {
        self.chat_history.push(msg);
    }
    
    pub fn get_chat_history(&self) -> &[ChatMessage] {
        &self.chat_history
    }
    
    pub fn update_clipboard(&mut self, clipboard: SharedClipboard) {
        self.clipboard = Some(clipboard);
    }
    
    pub fn get_clipboard(&self) -> Option<&SharedClipboard> {
        self.clipboard.as_ref()
    }
}

/// User cursor
#[derive(Debug, Clone)]
pub struct UserCursor {
    pub user_id: String,
    pub position: CursorPosition,
    pub selection: Option<SelectionRange>,
    pub timestamp: u64,
}

impl UserCursor {
    pub fn update_position(&mut self, position: CursorPosition) {
        self.position = position;
        self.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    pub fn update_selection(&mut self, selection: SelectionRange) {
        self.selection = Some(selection);
    }
}

/// Cursor position
#[derive(Debug, Clone, Default)]
pub struct CursorPosition {
    pub x: f32,
    pub y: f32,
}

impl CursorPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Chat message
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub user_id: String,
    pub user_name: String,
    pub message: String,
    pub timestamp: u64,
}

impl ChatMessage {
    pub fn new(user_id: String, user_name: String, message: String) -> Self {
        Self {
            user_id,
            user_name,
            message,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Shared clipboard
#[derive(Debug, Clone)]
pub struct SharedClipboard {
    pub content: String,
    pub user_id: String,
    pub timestamp: u64,
}

impl SharedClipboard {
    pub fn new(content: String, user_id: String) -> Self {
        Self {
            content,
            user_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Presence update
#[derive(Debug, Clone)]
pub struct PresenceUpdate {
    pub user_id: String,
    pub status: UserStatus,
    pub timestamp: u64,
}

/// Sync change
#[derive(Debug, Clone)]
pub struct SyncChange {
    pub user_id: String,
    pub path: String,
    pub change_type: ChangeType,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub timestamp: u64,
}

/// Collaboration manager
pub struct CollaborationManager {
    pub config: CollaborationConfig,
    pub session: Option<CollaborationSession>,
    pub local_user: Option<UserInfo>,
    pub local_cursor: Option<UserCursor>,
    pub connected_users: Vec<String>,
    pub disconnected_users: Vec<String>,
    pub is_connected: bool,
    pub is_host: bool,
    pub last_sync: u64,
    pub sync_counter: u64,
    pub pending_changes: Vec<SyncChange>,
    pub chat_buffer: Vec<ChatMessage>,
    pub presence_updates: Vec<PresenceUpdate>,
}

impl CollaborationManager {
    pub fn new(config: Option<CollaborationConfig>) -> Self {
        Self {
            config: config.unwrap_or_default(),
            session: None,
            local_user: None,
            local_cursor: None,
            connected_users: Vec::new(),
            disconnected_users: Vec::new(),
            is_connected: false,
            is_host: false,
            last_sync: 0,
            sync_counter: 0,
            pending_changes: Vec::new(),
            chat_buffer: Vec::new(),
            presence_updates: Vec::new(),
        }
    }

    /// Inicia sesión
    pub fn connect(&mut self, project_id: String, user_id: String, user_name: String) -> Option<String> {
        if self.session.is_some() {
            return None;
        }

        let session_id = uuid::Uuid::new_v4().to_string();
        let session_id_clone = session_id.clone();
        let mut session = CollaborationSession::new(session_id, project_id, user_id.clone());
        
        let mut user_info = UserInfo::new(user_id.clone(), user_name);
        user_info.set_owner();
        user_info.set_status(UserStatus::Online);
        session.add_user(user_id.clone(), user_info.clone());
        
        self.session = Some(session);
        self.local_user = Some(user_info);
        self.is_connected = true;
        self.is_host = true;
        
        Some(session_id_clone)
    }

    /// Conecta como guest
    pub fn connect_as_guest(&mut self, project_id: String, user_id: String) -> Option<String> {
        if self.session.is_some() {
            return None;
        }

        let session_id = uuid::Uuid::new_v4().to_string();
        let mut session = CollaborationSession::new(session_id.clone(), project_id, user_id.clone());
        
        let user_info = UserInfo::new(user_id.clone(), "Guest".to_string());
        session.add_user(user_id.clone(), user_info.clone());
        
        self.session = Some(session);
        self.local_user = Some(user_info);
        self.is_connected = true;
        self.is_host = false;
        
        Some(session_id)
    }

    /// Finaliza sesión
    pub fn disconnect(&mut self) {
        self.session.take();
        self.local_user = None;
        self.local_cursor = None;
        self.is_connected = false;
        self.is_host = false;
        self.connected_users.clear();
        self.disconnected_users.clear();
    }

    /// Agrega usuario remoto
    pub fn add_remote_user(&mut self, user_id: String, user_info: UserInfo) {
        if let Some(ref mut session) = self.session {
            session.add_user(user_id.clone(), user_info);
            self.connected_users.push(user_id);
        }
    }

    /// Elimina usuario remoto
    pub fn remove_remote_user(&mut self, user_id: &str) {
        if let Some(ref mut session) = self.session {
            session.remove_user(user_id);
            self.disconnected_users.push(user_id.to_string());
        }
    }

    /// Actualiza cursor local
    pub fn update_local_cursor(&mut self, position: CursorPosition, selection: Option<SelectionRange>) {
        if let Some(ref mut local_cursor) = self.local_cursor {
            local_cursor.update_position(position);
            if let Some(sel) = selection {
                local_cursor.update_selection(sel);
            }
        }
    }

    /// Agrega cursor remoto
    pub fn add_remote_cursor(&mut self, _user_id: String, cursor: UserCursor) {
        if let Some(ref mut session) = self.session {
            session.add_cursor(cursor);
        }
    }

    /// Elimina cursor remoto
    pub fn remove_remote_cursor(&mut self, user_id: &str) {
        if let Some(ref mut session) = self.session {
            session.cursors.remove(user_id);
        }
    }

    /// Envía mensaje de chat
    pub fn send_chat_message(&mut self, message: String) {
        if !self.config.enable_chat {
            return;
        }

        if let Some(ref local_user) = self.local_user {
            let chat_msg = ChatMessage::new(
                local_user.id.clone(),
                local_user.name.clone(),
                message
            );
            
            if let Some(ref mut session) = self.session {
                session.add_message(chat_msg);
            }
            
            // Limit history
            if self.config.max_history_messages > 0 {
                while self.session.as_ref().map(|s| s.chat_history.len()).unwrap_or(0) > self.config.max_history_messages {
                    self.session.as_mut().map(|s| s.chat_history.remove(0));
                }
            }
        }
    }

    /// Obtiene historial de chat
    pub fn get_chat_history(&self) -> &[ChatMessage] {
        self.session.as_ref().map(|s| s.get_chat_history()).unwrap_or(&[])
    }

    /// Obtiene chat buffer
    pub fn get_chat_buffer(&self) -> &[ChatMessage] {
        &self.chat_buffer
    }

    /// Agrega mensaje al buffer
    pub fn add_to_chat_buffer(&mut self, message: ChatMessage) {
        self.chat_buffer.push(message);
    }

    /// Obtiene clipboard
    pub fn get_clipboard(&self) -> Option<&SharedClipboard> {
        self.session.as_ref().and_then(|s| s.get_clipboard())
    }

    /// Actualiza clipboard
    pub fn update_clipboard(&mut self, content: String) {
        if let Some(ref local_user) = self.local_user {
            if self.config.enable_clipboard {
                if let Some(ref mut session) = self.session {
                    session.update_clipboard(SharedClipboard::new(content, local_user.id.clone()));
                }
            }
        }
    }

    /// Obtiene session
    pub fn session(&self) -> Option<&CollaborationSession> {
        self.session.as_ref()
    }

    /// Obtiene session mutable
    pub fn session_mut(&mut self) -> Option<&mut CollaborationSession> {
        self.session.as_mut()
    }

    /// Obtiene local user
    pub fn local_user(&self) -> Option<&UserInfo> {
        self.local_user.as_ref()
    }

    /// Obtiene local user mutable
    pub fn local_user_mut(&mut self) -> Option<&mut UserInfo> {
        self.local_user.as_mut()
    }

    /// Obtiene local cursor
    pub fn local_cursor(&self) -> Option<&UserCursor> {
        self.local_cursor.as_ref()
    }

    /// Obtiene local cursor mutable
    pub fn local_cursor_mut(&mut self) -> Option<&mut UserCursor> {
        self.local_cursor.as_mut()
    }

    /// Obtiene connected users
    pub fn connected_users(&self) -> &[String] {
        &self.connected_users
    }

    /// Obtiene disconnected users
    pub fn disconnected_users(&self) -> &[String] {
        &self.disconnected_users
    }

    /// Obtiene is connected
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    /// Obtiene is host
    pub fn is_host(&self) -> bool {
        self.is_host
    }

    /// Obtiene config
    pub fn config(&self) -> &CollaborationConfig {
        &self.config
    }

    /// Obtiene last sync
    pub fn last_sync(&self) -> u64 {
        self.last_sync
    }

    /// Obtiene sync counter
    pub fn sync_counter(&self) -> u64 {
        self.sync_counter
    }

    /// Obtiene pending changes
    pub fn pending_changes(&self) -> &[SyncChange] {
        &self.pending_changes
    }

    /// Agrega cambio pendiente
    pub fn add_pending_change(&mut self, change: SyncChange) {
        self.pending_changes.push(change);
    }

    /// Obtiene presence updates
    pub fn presence_updates(&self) -> &[PresenceUpdate] {
        &self.presence_updates
    }

    /// Agrega presence update
    pub fn add_presence_update(&mut self, update: PresenceUpdate) {
        self.presence_updates.push(update);
        // Limit updates
        if self.presence_updates.len() > 50 {
            self.presence_updates.remove(0);
        }
    }

    /// Obtiene user count
    pub fn user_count(&self) -> usize {
        self.session.as_ref().map(|s| s.user_count()).unwrap_or(0)
    }

    /// Obtiene cursor count
    pub fn cursor_count(&self) -> usize {
        self.session.as_ref().map(|s| s.get_cursors().len()).unwrap_or(0)
    }

    /// Obtiene chat message count
    pub fn chat_message_count(&self) -> usize {
        self.session.as_ref().map(|s| s.chat_history.len()).unwrap_or(0)
    }

    /// Obtiene clipboard user
    pub fn clipboard_user(&self) -> Option<&str> {
        self.session.as_ref().and_then(|s| s.get_clipboard()).map(|c| c.user_id.as_str())
    }

    /// Obtiene session ID
    pub fn session_id(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.session_id.as_str())
    }

    /// Obtiene project ID
    pub fn project_id(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.project_id.as_str())
    }

    /// Obtiene host ID
    pub fn host_id(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.host_id.as_str())
    }

    /// Obtiene created at
    pub fn created_at(&self) -> Option<u64> {
        self.session.as_ref().map(|s| s.created_at)
    }

    /// Obtiene owner
    pub fn owner(&self) -> Option<&UserInfo> {
        self.session.as_ref().and_then(|s| s.get_user(&s.host_id))
    }

    /// Obtiene is active
    pub fn is_active(&self) -> bool {
        self.session.is_some()
    }

    /// Obtiene last seen
    pub fn local_user_last_seen(&self) -> Option<u64> {
        self.local_user.as_ref().map(|u| u.last_seen)
    }

    /// Obtiene local user name
    pub fn local_user_name(&self) -> Option<&str> {
        self.local_user.as_ref().map(|u| u.name.as_str())
    }

    /// Obtiene local user avatar
    pub fn local_user_avatar(&self) -> Option<&str> {
        self.local_user.as_ref().and_then(|u| u.avatar.as_deref())
    }

    /// Obtiene local user joined at
    pub fn local_user_joined_at(&self) -> Option<u64> {
        self.local_user.as_ref().map(|u| u.joined_at)
    }

    /// Obtiene local user is owner
    pub fn local_user_is_owner(&self) -> Option<bool> {
        self.local_user.as_ref().map(|u| u.is_owner)
    }

    /// Obtiene local user status
    pub fn local_user_status(&self) -> Option<UserStatus> {
        self.local_user.as_ref().map(|u| u.status.clone())
    }

    /// Obtiene local cursor position
    pub fn local_cursor_position(&self) -> Option<CursorPosition> {
        self.local_cursor.as_ref().map(|c| CursorPosition {
            x: c.position.x,
            y: c.position.y,
        })
    }

    /// Obtiene local cursor selection
    pub fn local_cursor_selection(&self) -> Option<SelectionRange> {
        self.local_cursor.as_ref().and_then(|c| c.selection.as_ref()).map(|s| SelectionRange {
            start: s.start,
            end: s.end,
        })
    }

    /// Obtiene local cursor timestamp
    pub fn local_cursor_timestamp(&self) -> Option<u64> {
        self.local_cursor.as_ref().map(|c| c.timestamp)
    }
}

/// Collaboration config
#[derive(Debug, Clone)]
pub struct CollaborationConfig {
    pub sync_interval_ms: u64,
    pub max_sync_size: u64,
    pub chat_history_size: usize,
    pub enable_chat: bool,
    pub max_history_messages: usize,
    pub enable_clipboard: bool,
}

impl Default for CollaborationConfig {
    fn default() -> Self {
        Self {
            sync_interval_ms: 1000,
            max_sync_size: 1024 * 1024,
            chat_history_size: 100,
            enable_chat: true,
            max_history_messages: 100,
            enable_clipboard: true,
        }
    }
}

/// Collaboration session config
#[derive(Debug, Clone)]
pub struct CollaborationSessionConfig {
    pub auto_join: bool,
    pub require_invite: bool,
    pub max_users: usize,
}

impl Default for CollaborationSessionConfig {
    fn default() -> Self {
        Self {
            auto_join: false,
            require_invite: false,
            max_users: 100,
        }
    }
}

