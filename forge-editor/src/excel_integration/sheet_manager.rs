use crate::excel_integration::{
    ExcelSheet, Cell, Row, CellValue, CellType, DataType, Column,
    CellStyle, CellAlignHorizontal, CellAlignVertical, ChangeType, DateRange,
};
use std::collections::HashMap;

/// Gestor de hojas de cálculo
#[derive(Debug, Default)]
pub struct SheetManager {
    pub sheets: HashMap<String, ExcelSheet>,
    pub active_sheet: Option<String>,
    pub next_sheet_id: u64,
    pub next_row_id: u64,
}

impl SheetManager {
    /// Crear nuevo gestor de hojas
    pub fn new() -> Self {
        Self {
            sheets: HashMap::new(),
            active_sheet: None,
            next_sheet_id: 1,
            next_row_id: 1,
        }
    }
    
    /// Crear nueva hoja
    pub fn create_sheet(&mut self, name: &str) -> String {
        let sheet = ExcelSheet::new(crate::excel_integration::SheetName(name.to_string()));
        self.sheets.insert(name.to_string(), sheet);
        
        if self.active_sheet.is_none() {
            self.active_sheet = Some(name.to_string());
        }
        
        name.to_string()
    }
    
    /// Obtener hoja por nombre
    pub fn get_sheet(&self, name: &str) -> Option<&ExcelSheet> {
        self.sheets.get(name)
    }
    
    /// Obtener hoja mutante por nombre
    pub fn get_sheet_mut(&mut self, name: &str) -> Option<&mut ExcelSheet> {
        self.sheets.get_mut(name)
    }
    
    /// Setear hoja activa
    pub fn set_active_sheet(&mut self, name: &str) {
        self.active_sheet = Some(name.to_string());
    }
    
    /// Obtener hoja activa
    pub fn active_sheet(&self) -> Option<&str> {
        self.active_sheet.as_deref()
    }
    
    /// Obtener número total de hojas
    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }
    
    /// Obtener nombre de todas las hojas
    pub fn sheet_names(&self) -> Vec<String> {
        self.sheets.keys().cloned().collect()
    }
    
    /// Añadir fila a hoja
    pub fn add_row(&mut self, sheet_name: &str, row_data: Vec<Cell>) {
        if let Some(sheet) = self.sheets.get_mut(sheet_name) {
            let row = Row::new(row_data, sheet.row_count());
            sheet.add_row(row);
            sheet.last_modified = std::time::Instant::now();
        }
    }
    
    /// Escribir valor en celda
    pub fn write_cell(&mut self, sheet_name: &str, row: usize, col: usize, value: CellValue) {
        if let Some(sheet) = self.sheets.get_mut(sheet_name) {
            if let Some(row_mut) = sheet.get_row_mut(row) {
                if col >= row_mut.cells.len() {
                    for _ in row_mut.cells.len()..=col {
                        row_mut.cells.push(Cell::new(row, _, CellValue::Null, CellType::String));
                    }
                }
                row_mut.cells[col] = Cell::new(row, col, value, CellType::String);
                sheet.last_modified = std::time::Instant::now();
            } else {
                let mut new_row = Vec::new();
                for _ in 0..=col {
                    new_row.push(Cell::new(row, _, CellValue::Null, CellType::String));
                }
                let row = Row::new(new_row, row);
                sheet.add_row(row);
                
                let cell = Cell::new(row, col, value, CellType::String);
                sheet.get_row_mut(row).unwrap().cells[col] = cell;
                sheet.last_modified = std::time::Instant::now();
            }
        }
    }
    
    /// Obtener valor de celda
    pub fn get_cell(&self, sheet_name: &str, row: usize, col: usize) -> Option<&CellValue> {
        self.sheets.get(sheet_name)?.get_cell(row, col).map(|c| &c.value)
    }
    
    /// Obtener valor de columna por fila
    pub fn get_column_value(&self, sheet_name: &str, row: usize, column_name: &str) -> Option<&CellValue> {
        if let Some(sheet) = self.sheets.get(sheet_name) {
            if let Some(column) = sheet.get_column(column_name) {
                if let Some(row) = sheet.get_row(row) {
                    return row.cells.get(column.index).map(|c| &c.value);
                }
            }
        }
        None
    }
    
    /// Añadir columna a hoja
    pub fn add_column(&mut self, sheet_name: &str, column: Column) {
        if let Some(sheet) = self.sheets.get_mut(sheet_name) {
            sheet.add_column(column);
            sheet.last_modified = std::time::Instant::now();
        }
    }
    
    /// Obtener número total de filas
    pub fn total_rows(&self) -> usize {
        self.sheets.values().map(|s| s.row_count()).sum()
    }
    
    /// Obtener número total de columnas
    pub fn total_columns(&self) -> usize {
        self.sheets.values().map(|s| s.column_count()).max().unwrap_or(0)
    }
}

/// Validador de datos en hojas
#[derive(Debug, Default)]
pub struct DataValidator {
    pub validation_rules: HashMap<String, ValidationRule>,
    pub errors: Vec<ValidationError>,
}

impl DataValidator {
    /// Crear nuevo validador
    pub fn new() -> Self {
        Self {
            validation_rules: HashMap::new(),
            errors: Vec::new(),
        }
    }
    
    /// Añadir regla de validación por nombre de columna
    pub fn add_column_rule(&mut self, column_name: &str, rule: ValidationRule) {
        self.validation_rules.insert(column_name.to_string(), rule);
    }
    
    /// Validar fila completa
    pub fn validate_row(&self, sheet_name: &str, row: usize) -> Vec<ValidationError> {
        self.errors.clear();
        
        if let Some(sheet) = self.sheets.get(sheet_name) {
            if let Some(row_data) = sheet.get_row(row) {
                for cell in &row_data.cells {
                    if let Some(value) = &cell.value {
                        // Validar por tipo de dato
                        match &cell.cell_type {
                            CellType::Number => {
                                if let Some(_num) = value.as_number() {
                                    // Validaciones numéricas
                                    if let Some(rule) = self.validation_rules.get("numbers") {
                                        rule.validate(value, &mut self.errors);
                                    }
                                }
                            }
                            CellType::String => {
                                if let Some(_str) = value.as_string() {
                                    // Validaciones de texto
                                    if let Some(rule) = self.validation_rules.get("text") {
                                        rule.validate(value, &mut self.errors);
                                    }
                                }
                            }
                            CellType::Boolean => {
                                if let Some(_bool) = value.as_bool() {
                                    // Validaciones booleanas
                                    if let Some(rule) = self.validation_rules.get("booleans") {
                                        rule.validate(value, &mut self.errors);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        
        self.errors.clone()
    }
    
    /// Validar todas las filas
    pub fn validate_all(&self, sheet_name: &str) -> Vec<ValidationError> {
        let mut all_errors = Vec::new();
        
        if let Some(sheet) = self.sheets.get(sheet_name) {
            for row in 0..sheet.row_count() {
                let row_errors = self.validate_row(sheet_name, row);
                all_errors.extend(row_errors);
            }
        }
        
        all_errors
    }
}

/// Regla de validación
#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub data_type: DataType,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub required: bool,
    pub pattern: Option<String>,
    pub error_message: String,
}

impl ValidationRule {
    pub fn new(data_type: DataType, error_message: &str) -> Self {
        Self {
            data_type,
            min_value: None,
            max_value: None,
            required: false,
            pattern: None,
            error_message: error_message.to_string(),
        }
    }
    
    /// Setear valor mínimo
    pub fn with_min_value(mut self, min: f64) -> Self {
        self.min_value = Some(min);
        self
    }
    
    /// Setear valor máximo
    pub fn with_max_value(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }
    
    /// Setear como obligatorio
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// Setear patrón de validación
    pub fn with_pattern(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }
    
    /// Validar valor
    pub fn validate(&self, value: &CellValue, errors: &mut Vec<ValidationError>) {
        // Validar tipo de dato
        if !self.is_valid_type(value) {
            errors.push(ValidationError {
                column: "Unknown".to_string(),
                row: 0,
                message: format!("Invalid data type: expected {}", self.data_type),
                error_code: "INVALID_TYPE".to_string(),
            });
            return;
        }
        
        // Validar valor requerido
        if self.required && value.as_string().is_none() && value.as_number().is_none() {
            errors.push(ValidationError {
                column: "Unknown".to_string(),
                row: 0,
                message: format!("Required field is empty"),
                error_code: "REQUIRED".to_string(),
            });
            return;
        }
        
        // Validar rango numérico
        if let CellValue::Number(num) = value {
            if let Some(min) = self.min_value {
                if *num < min {
                    errors.push(ValidationError {
                        column: "Unknown".to_string(),
                        row: 0,
                        message: format!("Value must be at least {}", min),
                        error_code: "MIN_VALUE".to_string(),
                    });
                }
            }
            
            if let Some(max) = self.max_value {
                if *num > max {
                    errors.push(ValidationError {
                        column: "Unknown".to_string(),
                        row: 0,
                        message: format!("Value must be at most {}", max),
                        error_code: "MAX_VALUE".to_string(),
                    });
                }
            }
        }
        
        // Validar patrón
        if let Some(pattern) = &self.pattern {
            if let CellValue::String(s) = value {
                if !s.matches(pattern).next().is_some() {
                    errors.push(ValidationError {
                        column: "Unknown".to_string(),
                        row: 0,
                        message: format!("Value must match pattern: {}", pattern),
                        error_code: "PATTERN_MISMATCH".to_string(),
                    });
                }
            }
        }
    }
    
    /// Verificar si tipo es válido
    fn is_valid_type(&self, value: &CellValue) -> bool {
        match self.data_type {
            DataType::Number => value.as_number().is_some(),
            DataType::String => value.as_string().is_some(),
            DataType::Boolean => value.as_bool().is_some(),
            DataType::Unknown => true,
        }
    }
}

/// Error de validación
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub column: String,
    pub row: usize,
    pub message: String,
    pub error_code: String,
}

impl ValidationError {
    pub fn new(column: String, row: usize, message: String, error_code: String) -> Self {
        Self {
            column,
            row,
            message,
            error_code,
        }
    }
}
