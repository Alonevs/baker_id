//! # Hot Reload System - Improved
//! 
//! Sistema de hot reload mejorado con:
//! - File watcher automático
//! - Detección de cambios en scripts
//! - Hot reload en tiempo real
//! - UI mejorada para ver cambios
//! - Integración con Explorer Panel y Preview Panel

use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::SystemTime;
use crate::hot_reload::{HotReloadManager, ChangeType, PendingChange};
use crate::preview_panel::{PreviewPanel, AssetInfo, AssetType};

/// Sistema de Hot Reload mejorado
pub struct ImprovedHotReload {
    /// Manager de hot reload
    pub manager: HotReloadManager,
    /// Directorio a monitorear
    pub watch_dir: PathBuf,
    /// Contador de cambios
    pub change_count: u32,
    /// Último cambio
    pub last_change: Option<LastChangeInfo>,
    /// Estado del hot reload
    pub is_watching: bool,
    /// Assets modificados
    pub modified_assets: Vec<AssetInfo>,
}

impl Default for ImprovedHotReload {
    fn default() -> Self {
        Self::new()
    }
}

impl ImprovedHotReload {
    /// Crea nuevo sistema de hot reload
    pub fn new() -> Self {
        Self {
            manager: HotReloadManager::new(),
            watch_dir: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
            change_count: 0,
            last_change: None,
            is_watching: false,
            modified_assets: Vec::new(),
        }
    }

    /// Inicia el watcher
    pub fn start(&mut self) {
        if !self.is_watching {
            self.is_watching = true;
            println!("[HOT RELOAD] Started watching: {:?}", self.watch_dir);
        }
    }

    /// Detiene el watcher
    pub fn stop(&mut self) {
        self.is_watching = false;
        println!("[HOT RELOAD] Stopped watching");
    }

    /// Verifica cambios en el directorio
    pub fn check(&mut self) {
        if !self.is_watching {
            return;
        }

        let now = std::time::Instant::now();
        if now.duration_since(self.manager.file_watcher.as_ref().map(|w| w.last_check).unwrap_or(now)).as_secs() < 1 {
            return;
        }

        if let Some(ref mut watcher) = self.manager.file_watcher {
            watcher.check_for_changes();
            
            // Procesar cambios en el manager
            self.manager.process_pending_changes();
            self.manager.execute_pending_scripts();
            
            // Actualizar contador
            if !self.manager.pending_changes.is_empty() {
                self.change_count += 1;
                self.last_change = Some(LastChangeInfo {
                    file_path: self.manager.pending_changes.last()
                        .map(|c| c.file_path.clone())
                        .unwrap_or_default(),
                    change_type: self.manager.pending_changes.last()
                        .map(|c| c.change_type.clone())
                        .unwrap_or(ChangeType::Modified),
                });
                
                // Agregar asset modificado al preview panel
                if let Some(change) = self.manager.pending_changes.last() {
                    self.add_modified_asset(&change.file_path);
                }
            }
        }
    }

    /// Agrega asset modificado al preview panel
    fn add_modified_asset(&mut self, file_path: &str) {
        let path = PathBuf::from(file_path);
        if path.exists() {
            let asset_type = AssetInfo::detect_type(&path);
            let asset = AssetInfo::new(
                path.file_name().and_then(|n| n.to_str()).unwrap_or("Modified").to_string(),
                path,
                asset_type
            );
            self.modified_assets.push(asset);
        }
    }

    /// Maneja cambio manual
    pub fn handle_manual_change(&mut self, file_path: &str, change_type: ChangeType, content: Option<String>) {
        self.manager.register_change(
            file_path.to_string(),
            change_type,
            None,
            content
        );
        self.change_count += 1;
        self.last_change = Some(LastChangeInfo {
            file_path: file_path.to_string(),
            change_type,
        });
    }

    /// Obtiene stats del hot reload
    pub fn get_stats(&self) -> (u32, bool) {
        (self.change_count, self.is_watching)
    }

    /// Obtiene último cambio
    pub fn get_last_change(&self) -> Option<&LastChangeInfo> {
        self.last_change.as_ref()
    }

    /// Obtiene assets modificados
    pub fn get_modified_assets(&self) -> &[AssetInfo] {
        &self.modified_assets
    }

    /// Actualiza preview panel con cambios
    pub fn update_preview_panel(&mut self, preview_panel: &mut PreviewPanel) {
        for asset in &self.modified_assets {
            preview_panel.add_asset(asset.clone());
        }
    }
}

/// Información del último cambio
#[derive(Debug, Clone)]
pub struct LastChangeInfo {
    pub file_path: String,
    pub change_type: ChangeType,
}

impl std::fmt::Display for LastChangeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Last change: {:?} - {}",
            self.change_type, self.file_path
        )
    }
}

/// UI del Hot Reload mejorado
pub struct ImprovedHotReloadUI {
    /// Sistema de hot reload
    pub hot_reload: ImprovedHotReload,
    /// Controles
    pub show_reload_dialog: bool,
    /// Mensaje de estado
    pub status_message: String,
}

impl Default for ImprovedHotReloadUI {
    fn default() -> Self {
        Self::new()
    }
}

impl ImprovedHotReloadUI {
    /// Crea nueva UI
    pub fn new() -> Self {
        Self {
            hot_reload: ImprovedHotReload::new(),
            show_reload_dialog: false,
            status_message: String::new(),
        }
    }

    /// Renderiza UI del Hot Reload
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::ScrollArea::vertical().show(ctx, |ui| {
            ui.heading("🔥 Hot Reload");
            
            // Estado
            ui.horizontal(|ui| {
                ui.label(format!("Watching: {}", self.hot_reload.is_watching));
                ui.label(format!("Changes: {}", self.hot_reload.change_count));
            });
            
            if let Some(last_change) = self.hot_reload.get_last_change() {
                ui.label(format!("• {}", last_change));
            }
            
            ui.separator();
            
            // Controles
            ui.horizontal(|ui| {
                if ui.button("▶ Start Watching").clicked() {
                    self.hot_reload.start();
                    self.status_message = "Watching for changes...".to_string();
                }
                
                if ui.button("⏹ Stop Watching").clicked() {
                    self.hot_reload.stop();
                    self.status_message = "Stopped watching".to_string();
                }
                
                if ui.button("🔄 Manual Reload").clicked() {
                    self.hot_reload.manager.execute_pending_scripts();
                    self.status_message = "Reloaded manually".to_string();
                }
            });
            
            ui.add_space(10.0);
            
            // Mensajes
            if !self.status_message.is_empty() {
                ui.label(format!("ℹ️ {}", self.status_message));
            }
            
            ui.separator();
            
            // Conteo de cambios
            if self.hot_reload.change_count > 0 {
                ui.label(format!("📊 Total Changes: {}", self.hot_reload.change_count));
                
                // Lista de cambios recientes
                ui.label("Recent Changes:");
                for (i, change) in self.hot_reload.manager.pending_changes.iter().take(5).enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}.", i + 1));
                        ui.label(&change.file_path);
                        ui.label(format!("{:?}", change.change_type));
                    });
                }
            } else {
                ui.label("No changes detected yet");
            }
            
            ui.separator();
            
            // Modified Assets
            if !self.hot_reload.modified_assets.is_empty() {
                ui.label(format!("📁 Modified Assets: {}", self.hot_reload.modified_assets.len()));
                
                for asset in &self.hot_reload.modified_assets {
                    ui.horizontal(|ui| {
                        ui.label(format!("• {}", asset.name));
                        ui.label(format!("{:?}", asset.asset_type));
                    });
                }
            }
        });
    }
}
