//! UI modules

pub mod wizard;
pub mod panels;

use eframe::egui;
use crate::WizardState;

pub use wizard::show_wizard;
