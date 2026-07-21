use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AssetBrowser {
    pub assets: Vec<AssetItem>,
    pub current_path: PathBuf,
    pub selected_assets: Vec<Uuid>,
    pub current_filter: AssetFilter,
    pub is_open: bool,
    pub tree_view: Vec<TreeItem>,
}

#[derive(Debug, Clone)]
pub struct AssetItem {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub size: u64,
    pub modified: u64,
    pub is_loaded: bool,
    pub preview_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum AssetType {
    Scene,
    Sprite,
    Texture,
    Audio,
    Script,
    Model,
    Shader,
    Font,
    Video,
    Import,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct TreeItem {
    pub id: Uuid,
    pub name: String,
    pub is_folder: bool,
    pub children: Vec<TreeItem>,
    pub is_expanded: bool,
}

#[derive(Debug, Clone)]
pub enum AssetFilter {
    All,
    Scenes,
    Sprites,
    Audio,
    Scripts,
    Models,
    Shaders,
    Fonts,
    Videos,
}

impl AssetBrowser {
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
            current_path: PathBuf::from("."),
            selected_assets: Vec::new(),
            current_filter: AssetFilter::All,
            is_open: true,
            tree_view: Vec::new(),
        }
    }

    pub fn refresh(&mut self) {
        // Refrescar lista de assets
    }

    pub fn get_assets(&self) -> &Vec<AssetItem> {
        &self.assets
    }

    pub fn get_selected_assets(&self) -> &Vec<Uuid> {
        &self.selected_assets
    }

    pub fn add_asset(&mut self, asset: AssetItem) {
        self.assets.push(asset);
    }

    pub fn remove_asset(&mut self, id: &Uuid) {
        self.assets.retain(|a| a.id != *id);
    }

    pub fn select_asset(&mut self, id: Uuid) {
        if self.selected_assets.contains(&id) {
            self.selected_assets.retain(|aid| *aid != id);
        } else {
            self.selected_assets.push(id);
        }
    }

    pub fn filter_assets(&self) -> Vec<&AssetItem> {
        match self.current_filter {
            AssetFilter::All => self.assets.iter().collect(),
            AssetFilter::Scenes => self.assets.iter().filter(|a| matches!(a.asset_type, AssetType::Scene)).collect(),
            AssetFilter::Sprites => self.assets.iter().filter(|a| matches!(a.asset_type, AssetType::Sprite)).collect(),
            AssetFilter::Audio => self.assets.iter().filter(|a| matches!(a.asset_type, AssetType::Audio)).collect(),
            AssetFilter::Scripts => self.assets.iter().filter(|a| matches!(a.asset_type, AssetType::Script)).collect(),
            AssetFilter::Models => self.assets.iter().filter(|a| matches!(a.asset_type, AssetType::Model)).collect(),
            AssetFilter::Shaders => self.assets.iter().filter(|a| matches!(a.asset_type, AssetType::Shader)).collect(),
            AssetFilter::Fonts => self.assets.iter().filter(|a| matches!(a.asset_type, AssetType::Font)).collect(),
            AssetFilter::Videos => self.assets.iter().filter(|a| matches!(a.asset_type, AssetType::Video)).collect(),
        }
    }

    pub fn get_tree_view(&self) -> &Vec<TreeItem> {
        &self.tree_view
    }

    pub fn expand_folder(&mut self, item_id: &Uuid) {
        if let Some(item) = self.tree_view.iter_mut().find(|i| i.id == *item_id) {
            item.is_expanded = true;
        }
    }

    pub fn collapse_folder(&mut self, item_id: &Uuid) {
        if let Some(item) = self.tree_view.iter_mut().find(|i| i.id == *item_id) {
            item.is_expanded = false;
        }
    }
}

impl Default for AssetBrowser {
    fn default() -> Self {
        Self::new()
    }
}
