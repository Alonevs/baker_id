//! # Tile Palette Panel UI
//! 
//! Panel flotante de selección de Tiles para pintura sobre el lienzo 2D del editor.
//! Permite previsualizar los tiles troceados y seleccionar el tile activo para la herramienta TileMap.

use crate::sprite_slicer::{SpriteSlicer, SlicedTile};
use eframe::egui;

/// Panel de Paleta de Tiles interactivo
#[derive(Debug, Clone)]
pub struct TilePalettePanel {
    pub slicer: SpriteSlicer,
    pub selected_tile_index: usize,
    pub active_palette_name: String,
}

impl Default for TilePalettePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl TilePalettePanel {
    /// Crea un nuevo panel de paleta de tiles
    pub fn new() -> Self {
        Self {
            slicer: SpriteSlicer::new(),
            selected_tile_index: 0,
            active_palette_name: "Default_Terrain_Tileset".to_string(),
        }
    }

    /// Obtiene el tile actualmente seleccionado en la paleta
    pub fn get_selected_tile(&self) -> Option<&SlicedTile> {
        self.slicer.get_tile(self.selected_tile_index)
    }

    /// Selecciona el tile por índice
    pub fn select_tile(&mut self, index: usize) {
        if index < self.slicer.sliced_tiles.len() {
            self.selected_tile_index = index;
        }
    }

    /// Renderiza la UI del panel de Paleta de Tiles con egui
    pub fn ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🎨 Paleta de Tiles");
        ui.label(format!("Paleta activa: {}", self.active_palette_name));
        ui.separator();

        // Controles de ajuste de Grid de Troceado
        ui.collapsing("⚙️ Configuración de Grid", |ui| {
            let mut cols = self.slicer.config.columns;
            let mut rows = self.slicer.config.rows;
            let mut cell_w = self.slicer.config.cell_width;
            let mut cell_h = self.slicer.config.cell_height;

            let mut changed = false;
            ui.horizontal(|ui| {
                ui.label("Columnas:");
                if ui.add(egui::DragValue::new(&mut cols).range(1..=64)).changed() { changed = true; }
                ui.label("Filas:");
                if ui.add(egui::DragValue::new(&mut rows).range(1..=64)).changed() { changed = true; }
            });
            ui.horizontal(|ui| {
                ui.label("Ancho celda:");
                if ui.add(egui::DragValue::new(&mut cell_w).range(8..=256)).changed() { changed = true; }
                ui.label("Alto celda:");
                if ui.add(egui::DragValue::new(&mut cell_h).range(8..=256)).changed() { changed = true; }
            });

            if changed {
                self.slicer.update_grid_dimensions(cols, rows, cell_w, cell_h);
            }
        });

        ui.separator();

        // Información del tile seleccionado
        if let Some(tile) = self.get_selected_tile() {
            ui.group(|ui| {
                ui.label(format!("🎯 Tile Seleccionado: #{} ({})", tile.id, tile.name));
                ui.label(format!("Tamaño: {}x{} px | Pos: ({}, {})", tile.width, tile.height, tile.x, tile.y));
            });
        }

        ui.separator();
        ui.label("Seleccione un Tile para pintar:");

        // Grid interactiva de selección de Tiles
        egui::ScrollArea::vertical().show(ui, |ui| {
            let columns = self.slicer.config.columns as usize;
            let tiles = self.slicer.sliced_tiles.clone();

            egui::Grid::new("tile_palette_grid")
                .striped(true)
                .spacing([4.0, 4.0])
                .show(ui, |ui| {
                    for (idx, tile) in tiles.iter().enumerate() {
                        let is_selected = idx == self.selected_tile_index;
                        
                        let button_text = if is_selected {
                            format!("🟦 #{}", tile.id)
                        } else {
                            format!("⬜ #{}", tile.id)
                        };

                        if ui.button(button_text).clicked() {
                            self.selected_tile_index = idx;
                        }

                        if (idx + 1) % columns == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }
}
