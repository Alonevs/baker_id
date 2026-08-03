//! # Sprite & Sheet Slicer
//! 
//! Módulo de troceado de imágenes y spritesheets para generación de tilesets,
//! cálculo de coordenadas UV y metadatos de animación.

use std::path::PathBuf;

/// Estructura de un Tile troceado
#[derive(Debug, Clone, PartialEq)]
pub struct SlicedTile {
    pub id: usize,
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub uv_min: (f32, f32),
    pub uv_max: (f32, f32),
}

/// Configuración de Grid para el troceador
#[derive(Debug, Clone, PartialEq)]
pub struct TilesetConfig {
    pub columns: u32,
    pub rows: u32,
    pub cell_width: u32,
    pub cell_height: u32,
    pub margin: u32,
    pub spacing: u32,
}

impl Default for TilesetConfig {
    fn default() -> Self {
        Self {
            columns: 8,
            rows: 8,
            cell_width: 32,
            cell_height: 32,
            margin: 0,
            spacing: 0,
        }
    }
}

/// Motor de troceado de spritesheets
#[derive(Debug, Clone)]
pub struct SpriteSlicer {
    pub config: TilesetConfig,
    pub source_path: Option<PathBuf>,
    pub image_width: u32,
    pub image_height: u32,
    pub sliced_tiles: Vec<SlicedTile>,
}

impl Default for SpriteSlicer {
    fn default() -> Self {
        Self::new()
    }
}

impl SpriteSlicer {
    /// Crea un nuevo SpriteSlicer
    pub fn new() -> Self {
        let mut slicer = Self {
            config: TilesetConfig::default(),
            source_path: None,
            image_width: 256,
            image_height: 256,
            sliced_tiles: Vec::new(),
        };
        slicer.slice_grid();
        slicer
    }

    /// Carga información de origen de imagen
    pub fn set_image_source(&mut self, path: PathBuf, width: u32, height: u32) {
        self.source_path = Some(path);
        self.image_width = width.max(1);
        self.image_height = height.max(1);
        self.slice_grid();
    }

    /// Realiza el troceado por cuadrícula según la configuración
    pub fn slice_grid(&mut self) {
        self.sliced_tiles.clear();
        let mut tile_id = 0;

        let total_w = self.image_width as f32;
        let total_h = self.image_height as f32;

        for r in 0..self.config.rows {
            for c in 0..self.config.columns {
                let x = self.config.margin + c * (self.config.cell_width + self.config.spacing);
                let y = self.config.margin + r * (self.config.cell_height + self.config.spacing);

                let uv_min = (x as f32 / total_w, y as f32 / total_h);
                let uv_max = (
                    (x + self.config.cell_width) as f32 / total_w,
                    (y + self.config.cell_height) as f32 / total_h,
                );

                self.sliced_tiles.push(SlicedTile {
                    id: tile_id,
                    name: format!("Tile_{}", tile_id),
                    x,
                    y,
                    width: self.config.cell_width,
                    height: self.config.cell_height,
                    uv_min,
                    uv_max,
                });

                tile_id += 1;
            }
        }
    }

    /// Obtiene un tile por su ID
    pub fn get_tile(&self, id: usize) -> Option<&SlicedTile> {
        self.sliced_tiles.get(id)
    }

    /// Reconfigura el tamaño de celda y re-ejecuta el troceado
    pub fn update_grid_dimensions(&mut self, columns: u32, rows: u32, cell_w: u32, cell_h: u32) {
        self.config.columns = columns.max(1);
        self.config.rows = rows.max(1);
        self.config.cell_width = cell_w.max(1);
        self.config.cell_height = cell_h.max(1);
        self.slice_grid();
    }
}
