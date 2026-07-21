use std::path::PathBuf;
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::scene_node::{NodeData, Scene};
use crate::resource::Resource;

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub path: PathBuf,
    pub name: String,
    pub kind: AssetKind,
    pub metadata: HashMap<String, String>,
    pub preview: Option<PathBuf>,
    pub import_data: Option<ImportData>,
    pub is_loading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetKind {
    Sprite,
    Texture2D,
    Texture3D,
    AudioStream,
    AudioStreamPacked,
    AudioStreamGenerator,
    AudioEffect,
    AudioBus,
    AudioMixer,
    Animation,
    AnimationLibrary,
    Scene,
    SubResource,
    PackedScene,
    TileSet,
    TileMapLayer,
    TileMap,
    Mesh,
    MeshInstance,
    Light,
    Camera,
    Marker2D,
    Marker3D,
    Curve2D,
    Curve3D,
    CharacterBody2D,
    CharacterBody3D,
    Area2D,
    Area3D,
    StaticBody2D,
    StaticBody3D,
    Node2D,
    Node3D,
    Control,
    SubViewport,
    SubViewportContainer,
    CanvasLayer,
    MarginContainer,
    HBoxContainer,
    VBoxContainer,
    PanelContainer,
    ColorRect,
    Label,
    RichTextLabel,
    Button,
    TextureButton,
    ColorPickerButton,
    ProgressBar,
    TextureProgressBar,
    ScrollContainer,
    ScrollBar,
    TabBar,
    TabContainer,
    NinePatchRect,
    OptionButton,
    CheckBox,
    ButtonGroup,
    MenuButton,
    TooltipPopup,
    Popup,
    PopupMenu,
    Window,
    FileSystemControl,
    FileDialog,
    Tree,
    ItemList,
    ItemEdit,
    ListView,
    ListBox,
    ListContainer,
    GridContainer,
    BoxContainer,
    CenterContainer,
    AnchorLayout,
    MarginLayout,
    BoxLayout,
    AbsoluteAnchor,
    BoxLayoutContainer,
    HSplitContainer,
    VSplitContainer,
    Accordion,
    AccordionContainer,
    TextureRect,
    TextureButton,
    TextureProgressBar,
    Image,
    ImageTexture,
    RID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportData {
    pub source_path: PathBuf,
    pub imported_path: PathBuf,
    pub export_path: PathBuf,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub last_import_time: u64,
}

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("Asset not found: {0}")]
    AssetNotFound(String),
    #[error("Invalid asset format")]
    InvalidFormat,
    #[error("Asset is loading")]
    AssetLoading,
    #[error("Import failed: {0}")]
    ImportFailed(String),
    #[error("Export failed: {0}")]
    ExportFailed(String),
}

pub struct AssetLibrary {
    pub assets: HashMap<Uuid, Arc<Asset>>,
    pub asset_by_path: HashMap<PathBuf, Uuid>,
    pub asset_by_name: HashMap<String, Vec<Uuid>>,
    pub asset_by_kind: HashMap<AssetKind, Vec<Uuid>>,
    pub import_queue: Vec<PathBuf>,
    pub export_queue: Vec<PathBuf>,
}

impl AssetLibrary {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            asset_by_path: HashMap::new(),
            asset_by_name: HashMap::new(),
            asset_by_kind: HashMap::new(),
            import_queue: Vec::new(),
            export_queue: Vec::new(),
        }
    }

    pub fn add_asset(&mut self, asset: Arc<Asset>) -> Uuid {
        let id = asset.id;
        self.assets.insert(id, asset.clone());
        self.asset_by_path.insert(asset.path.clone(), id);
        self.asset_by_name
            .entry(asset.name.clone())
            .or_default()
            .push(id);
        self.asset_by_kind
            .entry(asset.kind.clone())
            .or_default()
            .push(id);
        id
    }

    pub fn get_asset(&self, id: &Uuid) -> Option<Arc<Asset>> {
        self.assets.get(id).cloned()
    }

    pub fn get_asset_by_path(&self, path: &PathBuf) -> Option<Arc<Asset>> {
        if let Some(&id) = self.asset_by_path.get(path) {
            self.assets.get(&id).cloned()
        } else {
            None
        }
    }

    pub fn get_asset_by_name(&self, name: &str) -> Vec<Arc<Asset>> {
        self.asset_by_name
            .get(name)
            .map(|ids| ids.iter().filter_map(|&id| self.assets.get(&id).cloned()).collect())
            .unwrap_or_default()
    }

    pub fn get_assets_by_kind(&self, kind: &AssetKind) -> Vec<Arc<Asset>> {
        self.asset_by_kind
            .get(kind)
            .map(|ids| ids.iter().filter_map(|&id| self.assets.get(&id).cloned()).collect())
            .unwrap_or_default()
    }

    pub fn queue_import(&mut self, path: &PathBuf) {
        self.import_queue.push(path.clone());
    }

    pub fn queue_export(&mut self, path: &PathBuf) {
        self.export_queue.push(path.clone());
    }

    pub fn process_imports(&mut self) {
        // Implementar importación de assets
    }

    pub fn process_exports(&mut self) {
        // Implementar exportación de assets
    }

    pub fn clear_queue(&mut self) {
        self.import_queue.clear();
        self.export_queue.clear();
    }
}

pub struct AssetManager {
    pub library: AssetLibrary,
    pub current_scene: Option<Arc<Scene>>,
    pub temp_assets: HashMap<Uuid, Arc<Asset>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            library: AssetLibrary::new(),
            current_scene: None,
            temp_assets: HashMap::new(),
        }
    }

    pub fn import_asset(&mut self, path: &PathBuf) -> Result<Arc<Asset>, AssetError> {
        let asset = Arc::new(Asset {
            id: Uuid::new_v4(),
            path: path.clone(),
            name: path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
            kind: self.detect_asset_kind(path)?,
            metadata: HashMap::new(),
            preview: None,
            import_data: None,
            is_loading: true,
        });

        self.library.add_asset(asset.clone());
        asset.import_data = Some(ImportData {
            source_path: path.clone(),
            imported_path: path.clone(),
            export_path: path.clone(),
            errors: Vec::new(),
            warnings: Vec::new(),
            last_import_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        });

        Ok(asset)
    }

    fn detect_asset_kind(&self, path: &PathBuf) -> Result<AssetKind, AssetError> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" => Ok(AssetKind::Texture2D),
            "mp3" | "ogg" | "wav" | "flac" => Ok(AssetKind::AudioStreamPacked),
            "anim" | "fnt" => Ok(AssetKind::Animation),
            "tscn" | "res" | "pck" => Ok(AssetKind::PackedScene),
            "tscn" => Ok(AssetKind::Scene),
            "mesh" => Ok(AssetKind::Mesh),
            "tset" => Ok(AssetKind::TileSet),
            _ => Ok(AssetKind::SubResource),
        }
    }

    pub fn export_asset(&mut self, asset_id: &Uuid) -> Result<PathBuf, AssetError> {
        if let Some(asset) = self.library.get_asset(asset_id) {
            let export_path = asset.path.with_extension("pck");
            // Implementar exportación
            Ok(export_path)
        } else {
            Err(AssetError::AssetNotFound(asset_id.to_string()))
        }
    }

    pub fn get_current_scene(&self) -> Option<Arc<Scene>> {
        self.current_scene.clone()
    }

    pub fn set_current_scene(&mut self, scene: Arc<Scene>) {
        self.current_scene = Some(scene);
    }
}

impl Default for AssetLibrary {
    fn default() -> Self {
        Self::new()
    }
}
