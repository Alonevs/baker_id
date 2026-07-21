use std::fs;
use std::collections::HashMap;

/// Parseador simple de CSV para diálogos
pub fn parse_csv_dialogs(path: &str) -> Result<HashMap<String, HashMap<String, Vec<String>>>, String> {
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

        let dialog_data: &mut HashMap<String, Vec<String>> = dialogs.entry(dialog_id.to_string()).or_insert_with(HashMap::new);
        dialog_data.entry(language.to_string()).or_insert_with(Vec::new);
        dialog_data.get_mut(&language.to_string()).unwrap().push(text.to_string());
    }

    Ok(dialogs)
}

/// Parseador simple de CSV para eventos
pub fn parse_csv_events(path: &str) -> Result<HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read CSV: {}", e))?;
    let mut events = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue;
        }

        let event_id = parts[0].trim();
        let event_type = parts[1].trim();

        if event_id.is_empty() {
            continue;
        }

        let event_data: &mut HashMap<String, HashMap<String, Vec<String>>> = events.entry(event_id.to_string()).or_insert(HashMap::new());

        // Inicializar HashMap anidados si no existe
        if let Some(event_types) = event_data.get_mut("types") {
            if let Some(vec) = event_types.get_mut(&event_type.to_string()) {
                vec.push(event_type.to_string());
            } else {
                event_types.insert(event_type.to_string(), vec![event_type.to_string()]);
            }
        } else {
            event_data.insert("types".to_string(), HashMap::new());
            let types = event_data.get_mut("types").unwrap();
            types.insert(event_type.to_string(), vec![event_type.to_string()]);
        }
    }

    Ok(events)
}
