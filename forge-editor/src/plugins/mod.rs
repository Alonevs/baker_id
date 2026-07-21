//! # Plugin Module
//! 
//! Sistema completo de plugins con:
//! - Plugin system
//! - Plugin loader
//! - Plugin hooks
//! - Plugin events
//! - Plugin UI
//! - Extension loader
//! - Plugin manager

pub mod plugin_system;
pub mod plugin_loader;
pub mod plugin_hooks;
pub mod plugin_events;
pub mod plugin_ui;
pub mod extension_loader;

pub use plugin_system::*;
pub use plugin_loader::*;
pub use plugin_hooks::*;
pub use plugin_events::*;
pub use plugin_ui::*;
pub use extension_loader::*;

