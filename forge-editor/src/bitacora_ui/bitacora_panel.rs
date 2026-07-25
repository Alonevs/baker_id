use crate::bitacora_ui::{BitacoraEntry, BitacoraFilter, BitacoraSeverity, BitacoraCategory};
use std::time::{Instant, Duration};
use crate::excel_integration::DateRange as ExcelDateRange;

/// Dirección de navegación en la lista
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationDirection {
    Previous,
    Next,
}

/// Tipo de notificación
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

impl Default for NotificationType {
    fn default() -> Self {
        Self::Info
    }
}

/// Notificación en UI
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u64,
    pub message: String,
    pub notification_type: NotificationType,
    pub created_at: Instant,
    pub duration: f32,
    pub is_dismissed: bool,
}

impl Notification {
    pub fn new(id: u64, message: &str, notification_type: NotificationType, duration: f32) -> Self {
        Self {
            id,
            message: message.to_string(),
            notification_type,
            created_at: Instant::now(),
            duration,
            is_dismissed: false,
        }
    }
    
    /// Obtener tiempo restante
    pub fn remaining_time(&self) -> f32 {
        self.duration - self.created_at.elapsed().as_secs_f32()
    }
    
    /// Verificar si notificación expiró
    pub fn is_expired(&self) -> bool {
        self.remaining_time() <= 0.0
    }
    
    /// Dismiss notificación
    pub fn dismiss(&mut self) {
        self.is_dismissed = true;
    }
}

/// Panel principal de Bitacora en UI
#[derive(Debug, Default)]
pub struct BitacoraPanel {
    pub entries_list: Vec<BitacoraEntry>,
    pub current_filter: BitacoraFilter,
    pub search_query: String,
    pub page: usize,
    pub items_per_page: usize,
    pub notifications: Vec<Notification>,
    pub is_open: bool,
    pub is_searching: bool,
    pub current_entry: Option<usize>,
    pub selected_category: Option<BitacoraCategory>,
    pub selected_severity: Option<BitacoraSeverity>,
    pub selected_date_range: Option<BitacoraDateRange>,
    pub created_at: Instant,
    pub last_updated: Instant,
}

impl BitacoraPanel {
    /// Crear nuevo panel de Bitacora
    pub fn new() -> Self {
        Self {
            entries_list: Vec::new(),
            current_filter: BitacoraFilter::All,
            search_query: String::new(),
            page: 0,
            items_per_page: 20,
            notifications: Vec::new(),
            is_open: false,
            is_searching: false,
            current_entry: None,
            selected_category: None,
            selected_severity: None,
            selected_date_range: None,
            created_at: Instant::now(),
            last_updated: Instant::now(),
        }
    }
    
    /// Abrir panel
    pub fn open(&mut self) {
        self.is_open = true;
        self.is_searching = false;
    }
    
    /// Cerrar panel
    pub fn close(&mut self) {
        self.is_open = false;
        self.current_entry = None;
    }
    
    /// Añadir entrada a la lista
    pub fn add_entry(&mut self, entry: BitacoraEntry) {
        self.entries_list.push(entry);
        self.last_updated = Instant::now();
        self.page = 0; // Resetear página
    }
    
    /// Añadir notificación
    pub fn add_notification(&mut self, message: &str, notification_type: NotificationType, duration: f32) {
        let id = self.notifications.len() as u64 + 1;
        let notification = Notification::new(id, message, notification_type, duration);
        self.notifications.push(notification);
    }
    
    /// Dismiss notificación
    pub fn dismiss_notification(&mut self, notification_id: u64) {
        if let Some(index) = self.notifications.iter().position(|n| n.id == notification_id) {
            self.notifications[index].dismiss();
        }
    }
    
    /// Filtrar entradas
    pub fn filter_entries(&mut self) {
        let filtered = self.entries_list.iter().filter(|entry| {
            // Filtro por categoría
            if let Some(category) = &self.selected_category {
                if entry.category != *category {
                    return false;
                }
            }
            
            // Filtro por severidad
            if let Some(severity) = &self.selected_severity {
                if entry.severity != *severity {
                    return false;
                }
            }
            
            // Filtro por rango de fechas
            if let Some(date_range) = &self.selected_date_range {
                if entry.timestamp < date_range.start || entry.timestamp > date_range.end {
                    return false;
                }
            }
            
            true
        }).cloned().collect::<Vec<_>>();
        
        self.entries_list = filtered;
        self.page = 0;
        self.is_searching = false;
    }
    
    /// Buscar entradas por palabra clave
    pub fn search_entries(&mut self, query: &str) {
        if query.is_empty() {
            self.search_query = String::new();
            self.is_searching = false;
            return;
        }
        
        self.search_query = query.to_string();
        self.is_searching = true;
        
        let filtered = self.entries_list.iter().filter(|entry| {
            let search_lower = query.to_lowercase();
            entry.message.to_lowercase().contains(&search_lower) ||
            entry.category.to_lowercase().contains(&search_lower) ||
            entry.player_name.to_lowercase().contains(&search_lower)
        }).cloned().collect::<Vec<_>>();
        
        self.entries_list = filtered;
        self.page = 0;
    }
    
    /// Navegar a página anterior
    pub fn navigate_previous(&mut self) {
        if self.page > 0 {
            self.page -= 1;
        }
    }
    
    /// Navegar a página siguiente
    pub fn navigate_next(&mut self) {
        let total_pages = (self.entries_list.len() + self.items_per_page - 1) / self.items_per_page;
        if self.page < total_pages - 1 {
            self.page += 1;
        }
    }
    
    /// Obtener entradas de la página actual
    pub fn get_page_entries(&self) -> &[BitacoraEntry] {
        let start = self.page * self.items_per_page;
        let end = (start + self.items_per_page).min(self.entries_list.len());
        &self.entries_list[start..end]
    }
    
    /// Obtener número total de páginas
    pub fn total_pages(&self) -> usize {
        if self.entries_list.is_empty() {
            return 0;
        }
        (self.entries_list.len() + self.items_per_page - 1) / self.items_per_page
    }
    
    /// Obtener número total de entradas
    pub fn total_entries(&self) -> usize {
        self.entries_list.len()
    }
    
    /// Obtener número de entradas en página actual
    pub fn entries_on_page(&self) -> usize {
        self.get_page_entries().len()
    }
    
    /// Obtener notificaciones no dismissidas
    pub fn active_notifications(&self) -> Vec<&Notification> {
        self.notifications.iter().filter(|n| !n.is_dismissed).collect()
    }
    
    /// Obtener notificaciones expiradas
    pub fn expired_notifications(&self) -> Vec<&Notification> {
        self.notifications.iter().filter(|n| n.is_expired()).collect()
    }
    
    /// Limpiar notificaciones expiradas
    pub fn cleanup_expired_notifications(&mut self) {
        self.notifications.retain(|n| !n.is_expired());
    }
    
    /// Limpiar todas las entradas
    pub fn clear_entries(&mut self) {
        self.entries_list.clear();
        self.page = 0;
    }
    
    /// Obtener tiempo de vida en segundos
    pub fn age(&self) -> f32 {
        self.created_at.elapsed().as_secs_f32()
    }
}

/// Fecha de rango para filtros
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitacoraDateRange {
    pub start: Instant,
    pub end: Instant,
}

impl BitacoraDateRange {
    pub fn new(start: Instant, end: Instant) -> Self {
        Self { start, end }
    }
    
    pub fn today() -> Self {
        let now = Instant::now();
        let start = now - Duration::days(1);
        Self { start, end: now }
    }
    
    pub fn week() -> Self {
        let now = Instant::now();
        let start = now - Duration::days(7);
        Self { start, end: now }
    }
    
    pub fn month() -> Self {
        let now = Instant::now();
        let start = now - Duration::days(30);
        Self { start, end: now }
    }
}
