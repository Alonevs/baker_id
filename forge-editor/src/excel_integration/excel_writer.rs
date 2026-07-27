use crate::excel_integration::{
    Workbook, ExcelSheet, Cell, Row, CellValue, CellType, CellStyle,
    CellAlignHorizontal, CellAlignVertical, DataType, Column, SheetName,
};

/// Escritor de Excel para crear y modificar libros
#[derive(Debug, Default)]
pub struct ExcelWriter {
    pub workbook: Workbook,
    pub current_sheet: Option<SheetName>,
    pub pending_changes: Vec<PendingChange>,
}

impl ExcelWriter {
    /// Crear nuevo escritor de Excel
    pub fn new() -> Self {
        Self {
            workbook: Workbook::new(),
            current_sheet: None,
            pending_changes: Vec::new(),
        }
    }
    
    /// Crear nuevo libro de trabajo
    pub fn create_workbook(&mut self) {
        self.workbook = Workbook::new();
    }
    
    /// Crear nueva hoja
    pub fn create_sheet(&mut self, sheet_name: &str) -> SheetName {
        let sheet = ExcelSheet::new(SheetName(sheet_name.to_string()));
        self.workbook.add_sheet(sheet);
        
        if self.current_sheet.is_none() {
            self.current_sheet = Some(SheetName(sheet_name.to_string()));
        }
        
        SheetName(sheet_name.to_string())
    }
    
    /// Setear hoja actual
    pub fn set_current_sheet(&mut self, sheet_name: &str) {
        self.current_sheet = Some(SheetName(sheet_name.to_string()));
    }
    
    /// Obtener hoja actual
    pub fn current_sheet(&self) -> Option<&SheetName> {
        self.current_sheet.as_ref()
    }
    
    /// Obtener hoja mutante actual
    pub fn current_sheet_mut(&mut self) -> Option<&mut SheetName> {
        self.current_sheet.as_mut()
    }
    
    /// Escribir valor en celda
    pub fn write_cell(
        &mut self,
        sheet_name: &str,
        row: usize,
        col: usize,
        value: CellValue,
    ) {
        if let Some(sheet) = self.workbook.get_sheet_mut(sheet_name) {
            if let Some(row_mut) = sheet.get_row_mut(row) {
                if col >= row_mut.cells.len() {
                    // Añadir celdas vacías hasta llegar a la columna
                    for _ in row_mut.cells.len()..=col {
                        row_mut.cells.push(Cell::new(row, row_mut.cells.len(), CellValue::Null, CellType::String));
                    }
                }
                
                row_mut.cells[col] = Cell::new(row, col, value, CellType::String);
                sheet.last_modified = std::time::Instant::now();
            } else {
                // Añadir nueva fila
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
    
    /// Escribir texto en celda
    pub fn write_string(&mut self, sheet_name: &str, row: usize, col: usize, text: &str) {
        self.write_cell(sheet_name, row, col, CellValue::String(text.to_string()));
    }
    
    /// Escribir número en celda
    pub fn write_number(&mut self, sheet_name: &str, row: usize, col: usize, number: f64) {
        self.write_cell(sheet_name, row, col, CellValue::Number(number));
    }
    
    /// Escribir booleano en celda
    pub fn write_bool(&mut self, sheet_name: &str, row: usize, col: usize, boolean: bool) {
        self.write_cell(sheet_name, row, col, CellValue::Boolean(boolean));
    }
    
    /// Escribir rango de valores
    pub fn write_range(
        &mut self,
        sheet_name: &str,
        range: CellRange,
        data: &[Vec<String>],
    ) {
        for (row_idx, row_data) in data.iter().enumerate() {
            let row = range.start_row + row_idx;
            for (col_idx, value) in row_data.iter().enumerate() {
                let col = range.start_col + col_idx;
                if col < range.end_col {
                    self.write_string(sheet_name, row, col, value);
                }
            }
        }
    }
    
    /// Escribir celda por nombre de columna
    pub fn write_column_value(
        &mut self,
        sheet_name: &str,
        row: usize,
        column_name: &str,
        value: CellValue,
    ) {
        if let Some(sheet) = self.workbook.get_sheet_mut(sheet_name) {
            if let Some(column) = sheet.get_column_mut(column_name) {
                if let Some(row_mut) = sheet.get_row_mut(row) {
                    if column.index >= row_mut.cells.len() {
                        for _ in row_mut.cells.len()..=column.index {
                            row_mut.cells.push(Cell::new(row, _, CellValue::Null, CellType::String));
                        }
                    }
                    row_mut.cells[column.index] = Cell::new(row, column.index, value, column.data_type);
                    sheet.last_modified = std::time::Instant::now();
                } else {
                    row_mut.cells[column.index] = Cell::new(row, column.index, value, column.data_type);
                    sheet.last_modified = std::time::Instant::now();
                }
            }
        }
    }
    
    /// Aplicar estilo a celda
    pub fn apply_style(
        &mut self,
        sheet_name: &str,
        row: usize,
        col: usize,
        style: CellStyle,
    ) {
        if let Some(sheet) = self.workbook.get_sheet_mut(sheet_name) {
            if let Some(row_mut) = sheet.get_row_mut(row) {
                if col < row_mut.cells.len() {
                    row_mut.cells[col].style = style;
                    sheet.last_modified = std::time::Instant::now();
                }
            }
        }
    }
    
    /// Aplicar estilo a rango
    pub fn apply_style_range(
        &mut self,
        sheet_name: &str,
        range: CellRange,
        style: CellStyle,
    ) {
        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                self.apply_style(sheet_name, row, col, style);
            }
        }
    }
    
    /// Ocultar columna
    pub fn hide_column(&mut self, sheet_name: &str, column_index: usize) {
        if let Some(sheet) = self.workbook.get_sheet_mut(sheet_name) {
            if let Some(column) = sheet.get_column_mut(&format!("Col{}", column_index)) {
                column.is_hidden = true;
                sheet.last_modified = std::time::Instant::now();
            }
        }
    }
    
    /// Mostrar columna
    pub fn show_column(&mut self, sheet_name: &str, column_index: usize) {
        if let Some(sheet) = self.workbook.get_sheet_mut(sheet_name) {
            if let Some(column) = sheet.get_column_mut(&format!("Col{}", column_index)) {
                column.is_hidden = false;
                sheet.last_modified = std::time::Instant::now();
            }
        }
    }
    
    /// Ocultar hoja
    pub fn hide_sheet(&mut self, sheet_name: &str) {
        if let Some(sheet) = self.workbook.get_sheet_mut(sheet_name) {
            sheet.is_hidden = true;
            self.last_modified = std::time::Instant::now();
        }
    }
    
    /// Mostrar hoja
    pub fn show_sheet(&mut self, sheet_name: &str) {
        if let Some(sheet) = self.workbook.get_sheet_mut(sheet_name) {
            sheet.is_hidden = false;
            self.last_modified = std::time::Instant::now();
        }
    }
    
    /// Obtener número de cambios pendientes
    pub fn pending_changes_count(&self) -> usize {
        self.pending_changes.len()
    }
    
    /// Limpiar cambios pendientes
    pub fn clear_pending_changes(&mut self) {
        self.pending_changes.clear();
    }
    
    /// Exportar Bitacora a Excel
    pub fn export_bitacora(
        &mut self,
        manager: &crate::bitacora_manager::BitacoraManager,
        filename: &str,
    ) -> Result<(), String> {
        // Crear nuevo libro
        self.workbook = Workbook::new();
        
        // Crear hoja "Bitacora"
        let sheet_name = SheetName("Bitacora".to_string());
        let sheet = ExcelSheet::new(sheet_name.clone());
        self.workbook.add_sheet(sheet);
        self.current_sheet = Some(sheet_name.clone());
        
        // Escribir encabezados
        let headers = vec![
            "ID", "Texto", "Enlace", "Relacionado",
        ];
        for (col, header) in headers.iter().enumerate() {
            self.write_string("Bitacora", 0, col, header);
        }
        
        // Escribir datos de cada entrada
        let entries = manager.get_entries_for_ui();
        for (row, entry) in entries.iter().enumerate() {
            let row_num = row + 1; // Saltar encabezado
            
            // ID
            self.write_string("Bitacora", row_num, 0, &entry.id);
            
            // Texto
            self.write_string("Bitacora", row_num, 1, &entry.text);
            
            // Enlaces (serializados)
            let links_str: Vec<String> = entry.links.iter().map(|link| match link {
                crate::bitacora_manager::LinkType::Event(id) => format!("Event({})", id),
                crate::bitacora_manager::LinkType::Dialog(id) => format!("Dialog({})", id),
                crate::bitacora_manager::LinkType::Actor(id) => format!("Actor({})", id),
                crate::bitacora_manager::LinkType::Variable(id) => format!("Variable({})", id),
                crate::bitacora_manager::LinkType::Scene(id) => format!("Scene({})", id),
                crate::bitacora_manager::LinkType::Note(id) => format!("Note({})", id),
                crate::bitacora_manager::LinkType::Unknown(id) => format!("Unknown({})", id),
            }).collect();
            self.write_string("Bitacora", row_num, 2, &links_str.join(", "));
            
            // Relacionado
            self.write_string("Bitacora", row_num, 3, entry.related_to.as_deref().unwrap_or(""));
        }
        
        println!("[EXCEL] Bitacora exportada a: {}", filename);
        Ok(())
    }
}

/// Cambio pendiente
#[derive(Debug, Clone)]
pub struct PendingChange {
    pub change_type: ChangeType,
    pub sheet_name: String,
    pub row: usize,
    pub col: usize,
    pub value: CellValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeType {
    Set,
    Delete,
    Style,
}
