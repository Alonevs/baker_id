use crate::bitacora_ui::BitacoraDateRange;

/// Categoría de entrada de Bitacora
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitacoraCategory {
    Game,
    Network,
    Player,
    System,
    Error,
    Warning,
    Info,
}

impl Default for BitacoraCategory {
    fn default() -> Self {
        Self::Game
    }
}

impl std::fmt::Display for BitacoraCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Game => write!(f, "Game"),
            Self::Network => write!(f, "Network"),
            Self::Player => write!(f, "Player"),
            Self::System => write!(f, "System"),
            Self::Error => write!(f, "Error"),
            Self::Warning => write!(f, "Warning"),
            Self::Info => write!(f, "Info"),
        }
    }
}

/// Severidad de entrada de Bitacora
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitacoraSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Default for BitacoraSeverity {
    fn default() -> Self {
        Self::Info
    }
}

impl std::fmt::Display for BitacoraSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "Critical"),
            Self::High => write!(f, "High"),
            Self::Medium => write!(f, "Medium"),
            Self::Low => write!(f, "Low"),
            Self::Info => write!(f, "Info"),
        }
    }
}

/// Entrada de Bitacora
#[derive(Debug, Clone)]
pub struct BitacoraEntry {
    pub id: u64,
    pub timestamp: std::time::Instant,
    pub category: BitacoraCategory,
    pub severity: BitacoraSeverity,
    pub message: String,
    pub player_name: String,
    pub player_id: u64,
    pub metadata: std::collections::HashMap<String, String>,
}

impl BitacoraEntry {
    pub fn new(
        id: u64,
        timestamp: std::time::Instant,
        category: BitacoraCategory,
        severity: BitacoraSeverity,
        message: &str,
        player_name: &str,
        player_id: u64,
    ) -> Self {
        Self {
            id,
            timestamp,
            category,
            severity,
            message: message.to_string(),
            player_name: player_name.to_string(),
            player_id,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// Añadir metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    
    /// Obtener metadata por clave
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Filtro para Bitacora
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitacoraFilter {
    All,
    ByCategory(BitacoraCategory),
    BySeverity(BitacoraSeverity),
    ByDateRange(BitacoraDateRange),
    ByPlayer(u64),
    ByKeyword(&'static str),
}

impl Default for BitacoraFilter {
    fn default() -> Self {
        Self::All
    }
}

impl BitacoraFilter {
    /// Crear filtro por categoría
    pub fn by_category(category: BitacoraCategory) -> Self {
        Self::ByCategory(category)
    }
    
    /// Crear filtro por severidad
    pub fn by_severity(severity: BitacoraSeverity) -> Self {
        Self::BySeverity(severity)
    }
    
    /// Crear filtro por rango de fechas
    pub fn by_date_range(date_range: BitacoraDateRange) -> Self {
        Self::ByDateRange(date_range)
    }
    
    /// Crear filtro por jugador
    pub fn by_player(player_id: u64) -> Self {
        Self::ByPlayer(player_id)
    }
    
    /// Crear filtro por palabra clave
    pub fn by_keyword(keyword: &'static str) -> Self {
        Self::ByKeyword(keyword)
    }
}

/// Resultado de búsqueda
#[derive(Debug, Clone)]
pub struct SearchResults {
    pub matches: Vec<BitacoraEntry>,
    pub total_matches: usize,
    pub search_time: f32,
}

impl SearchResults {
    pub fn new(matches: Vec<BitacoraEntry>, total_matches: usize, search_time: f32) -> Self {
        Self {
            matches,
            total_matches,
            search_time,
        }
    }
}

