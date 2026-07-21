use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::{Arc, RwLock};

/// Sistema de lectura/escritura de Excel (xlsx) y CSV
/// Soporta actualización en tiempo real
pub struct ExcelManager {
    pub excel_path: Option<String>,
    pub csv_path: Option<String>,
    pub events: Arc<RwLock<HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>>>,
    pub dialogs: Arc<RwLock<HashMap<String, HashMap<String, Vec<String>>>>>,
    pub current_language: String,
    pub is_loading: bool,
}

impl Default for ExcelManager {
    fn default() -> Self {
        Self {
            excel_path: None,
            csv_path: None,
            events: Arc::new(RwLock::new(HashMap::new())),
            dialogs: Arc::new(RwLock::new(HashMap::new())),
            current_language: "es".to_string(),
            is_loading: false,
        }
    }
}

impl ExcelManager {
    /// Iniciar gestión de Excel/CSV
    pub fn init(&mut self, path: Option<String>) {
        self.excel_path = path.clone();
        self.csv_path = path;
        self.is_loading = true;
    }

    /// Cargar datos de Excel o CSV
    pub fn load(&mut self) -> Result<(), String> {
        match (self.excel_path.clone(), self.csv_path.clone()) {
            (Some(path), _) => {
                return self.load_excel(&path);
            }
            (_, Some(path)) => {
                return self.load_csv(&path);
            }
            _ => {
                return Err("No path specified".to_string());
            }
        }
    }

    /// Cargar datos de Excel (.xlsx) - Placeholder
    fn load_excel(&self, path: &str) -> Result<(), String> {
        // parse_csv_dialogs devuelve HashMap<String, HashMap<String, Vec<String>>>
        let dialogs = crate::csv_parser::parse_csv_dialogs(path)?;
        self.dialogs.write().unwrap().insert(path.to_string(), dialogs);
        Ok(())
    }

    /// Crear archivo temporal para live updates
    pub fn create_temp_file(&self, original_path: &str) -> Result<String, String> {
        use std::fs::File;
        use std::io::Write;
        
        let ext = if original_path.ends_with(".xlsx") { "xlsx" } else if original_path.ends_with(".csv") { "csv" } else { "csv" };
        
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("forge_temp_{}.{}", std::process::id(), ext));
        
        // Copiar archivo original a temporal
        if let Ok(mut file) = File::open(original_path) {
            let mut temp_file = match File::create(&temp_path) {
                Ok(f) => f,
                Err(e) => return Err(format!("Failed to create temp file: {}", e)),
            };
            if let Err(e) = std::io::copy(&mut file, &mut temp_file) {
                return Err(format!("Failed to copy file: {}", e));
            }
        } else {
            return Err(format!("Failed to open original file: {}", original_path));
        }
        
        Ok(temp_path.to_string_lossy().to_string())
    }

    /// Guardar cambios en archivo temporal
    pub fn save_to_temp(&self, temp_path: &str) -> Result<(), String> {
        let events = self.events.read().unwrap();
        let dialogs = self.dialogs.read().unwrap();
        
        // Leer contenido temporal y actualizar
        if let Ok(content) = std::fs::read_to_string(temp_path) {
            self.events.write().unwrap().clear();
            self.dialogs.write().unwrap().clear();
            
            let events = crate::csv_parser::parse_csv_events(&content)?;
            let dialogs = crate::csv_parser::parse_csv_dialogs(&content)?;

            let events_converted: HashMap<String, HashMap<String, Vec<String>>> = events.iter()
                .map(|(event_id, event_data)| {
                    let mut event_converted: HashMap<String, Vec<String>> = HashMap::new();
                    let triggers_list = event_converted.entry("triggers".to_string()).or_insert_with(Vec::new);
                    if let Some(types) = event_data.get("types") {
                        for (trigger_type, triggers) in types {
                            for t in triggers {
                                triggers_list.push(t.clone());
                            }
                        }
                    }
                    (event_id.clone(), event_converted)
                })
                .collect();
            let dialogs_converted: HashMap<String, HashMap<String, Vec<String>>> = dialogs.iter()
                .map(|(dialog_id, texts_data)| {
                    let mut texts_data_converted: HashMap<String, Vec<String>> = HashMap::new();
                    for lang in texts_data.keys() {
                        texts_data_converted.insert(lang.clone(), texts_data.get(lang).cloned().unwrap_or_default());
                    }
                    (dialog_id.clone(), texts_data_converted)
                })
                .collect();
            self.events.write().unwrap().insert(temp_path.to_string(), events_converted);
            self.dialogs.write().unwrap().insert(temp_path.to_string(), dialogs_converted);
        }
        
        Ok(())
    }

    /// Cargar datos de CSV
    fn load_csv(&mut self, path: &str) -> Result<(), String> {
        let events = self.parse_csv_events(path)?;
        let dialogs = self.parse_csv_dialogs(path)?;

        // Guardar en memoria
        {
            let mut events_lock = self.events.write().unwrap();
            *events_lock = events;
        }
        {
            let mut dialogs_lock = self.dialogs.write().unwrap();
            *dialogs_lock = dialogs;
        }

        self.is_loading = false;
        Ok(())
    }

    /// Parsear eventos de CSV
    fn parse_csv_events(&self, path: &str) -> Result<HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read CSV: {}", e))?;
        let mut events = HashMap::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 5 {
                continue;
            }

            let event_id = parts[0].trim();
            let trigger = parts[1].trim();
            let dialog_id = parts[2].trim();
            let language = parts[3].trim();
            let text = parts[4].trim();

            if event_id.is_empty() || trigger.is_empty() || dialog_id.is_empty() {
                continue;
            }

            // Triggers
            events.entry(event_id.to_string()).or_insert_with(|| HashMap::new());
            let mut event_data = events.get_mut(&event_id.to_string()).unwrap();
            event_data.entry("triggers".to_string()).or_insert_with(HashMap::new);
            let mut triggers = event_data.get_mut("triggers").unwrap();
            triggers.entry("list".to_string()).or_insert_with(Vec::new);
            triggers.get_mut("list").unwrap().push(trigger.to_string());

            // Dialog_ID
            event_data.entry("dialog".to_string()).or_insert_with(HashMap::new);
            let mut dialog = event_data.get_mut("dialog").unwrap();
            dialog.entry("list".to_string()).or_insert_with(Vec::new);
            dialog.get_mut("list").unwrap().push(dialog_id.to_string());

            // Text por idioma
            event_data.entry("texts".to_string()).or_insert_with(HashMap::new);
            let mut texts = event_data.get_mut("texts").unwrap();
            texts.entry(language.to_string()).or_insert_with(Vec::new);
            texts.get_mut(&language.to_string()).unwrap().push(text.to_string());
        }

        Ok(events)
    }

    /// Parsear diálogos de CSV
    fn parse_csv_dialogs(&self, path: &str) -> Result<HashMap<String, HashMap<String, Vec<String>>>, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read CSV: {}", e))?;
        let mut dialogs = HashMap::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 3 {
                continue;
            }

            let dialog_id = parts[0].trim();
            let language = parts[1].trim();
            let text = parts[2].trim();

            if dialog_id.is_empty() || language.is_empty() {
                continue;
            }

            let dialog_data = dialogs.entry(dialog_id.to_string()).or_insert_with(HashMap::new);
            dialog_data.entry(language.to_string()).or_insert_with(Vec::new);
            dialog_data.get_mut(language).unwrap().push(text.to_string());
        }

        Ok(dialogs)
    }

    /// Obtener evento por ID
    pub fn get_event(&self, event_id: &str) -> Option<HashMap<String, HashMap<String, Vec<String>>>> {
        let events = self.events.read().unwrap();
        events.get(event_id).cloned()
    }

    /// Obtener diálogo por ID
    pub fn get_dialog(&self, dialog_id: &str) -> Option<HashMap<String, Vec<String>>> {
        let dialogs = self.dialogs.read().unwrap();
        dialogs.get(dialog_id).cloned()
    }

    /// Obtener texto de evento en idioma específico
    pub fn get_event_text(&self, event_id: &str, language: &str) -> Option<String> {
        let events = self.events.read().unwrap();
        let event = events.get(event_id)?;
        let triggers = event.get("triggers")?;
        let triggers_list = triggers.get("list")?;
        triggers_list.first().map(|s| s.as_str().to_string())
    }

    /// Obtener texto de diálogo en idioma específico
    pub fn get_dialog_text(&self, dialog_id: &str, language: &str) -> Option<String> {
        let dialogs = self.dialogs.read().unwrap();
        let dialog = dialogs.get(dialog_id)?;
        dialog.get(language).cloned().and_then(|texts| texts.first().cloned())
    }

    /// Guardar cambios en Excel
    pub fn save_excel(&self, path: &str) -> Result<(), String> {
        // Simplificar: guardar como CSV
        let mut content = String::new();
        
        // Eventos
        content.push_str("ID,Triggers,Dialog_ID,Actions\n");
        for (event_id, event_data) in self.events.read().unwrap().iter() {
            let triggers = event_data.get("triggers")
                .and_then(|t| t.get("list"))
                .map(|l| l.iter().map(|s| s.clone()).collect::<Vec<String>>())
                .unwrap_or_default();

            let dialog_id = event_data.get("dialog")
                .and_then(|d| d.get("list"))
                .map(|l| l.iter().map(|s| s.clone()).collect::<Vec<String>>())
                .unwrap_or_default();

            let actions = event_data.get("actions")
                .and_then(|a| a.get("list"))
                .map(|l| l.iter().map(|s| s.clone()).collect::<Vec<String>>())
                .unwrap_or_default();

            content.push_str(&format!("{},{},{},{}\n", 
                event_id, 
                triggers.join(","), 
                dialog_id.join(","), 
                actions.join(",")));
        }

        // Diálogos
        content.push_str("ID,Text\n");
        for (dialog_id, dialog_data) in self.dialogs.read().unwrap().iter() {
            let text = dialog_data.get(&self.current_language.to_string())
                .and_then(|texts| texts.first())
                .cloned()
                .unwrap_or_default();

            content.push_str(&format!("{},{}\n", dialog_id, text));
        }

        // Escribir archivo
        std::fs::write(path, &content).map_err(|e| format!("Failed to save file: {}", e))
    }
}
