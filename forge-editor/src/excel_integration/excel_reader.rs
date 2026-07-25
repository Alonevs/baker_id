use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

/// Nombre de hoja en un libro de Excel
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SheetName(pub String);

impl SheetName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

/// Rango de celdas en una hoja
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellRange {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

/// Rango de fechas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    pub start: SystemTime,
    pub end: SystemTime,
}

impl DateRange {
    pub fn new(start: SystemTime, end: SystemTime) -> Self {
        Self { start, end }
    }
}

impl CellRange {
    pub fn new(start_row: usize, start_col: usize, end_row: usize, end_col: usize) -> Self {
        Self {
            start_row,
            start_col,
            end_row,
            end_col,
        }
    }
    
    /// Obtener número de filas
    pub fn rows(&self) -> usize {
        self.end_row - self.start_row + 1
    }
    
    /// Obtener número de columnas
    pub fn cols(&self) -> usize {
        self.end_col - self.start_col + 1
    }
    
    /// Obtener número total de celdas
    pub fn cells(&self) -> usize {
        self.rows() * self.cols()
    }
}

/// Valor de celda
#[derive(Debug, Clone)]
pub enum CellValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Date(SystemTime),
    Null,
}

impl CellValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_date(&self) -> Option<SystemTime> {
        match self {
            Self::Date(d) => Some(*d),
            _ => None,
        }
    }
}

/// Tipo de celda
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    String,
    Number,
    Boolean,
    Date,
    Formula,
}

/// Estilo de celda
#[derive(Debug, Clone, Copy, Default)]
pub struct CellStyle {
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_underlined: bool,
    pub is_hidden: bool,
    pub align_horizontal: CellAlignHorizontal,
    pub align_vertical: CellAlignVertical,
    pub wrap_text: bool,
}

/// Alineación horizontal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellAlignHorizontal {
    #[default]
    Left,
    Center,
    Right,
}

/// Alineación vertical
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellAlignVertical {
    #[default]
    Top,
    Center,
    Bottom,
}

/// Celda en una hoja
#[derive(Debug, Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub value: CellValue,
    pub cell_type: CellType,
    pub style: CellStyle,
    pub formula: Option<String>,
}

impl Cell {
    pub fn new(row: usize, col: usize, value: CellValue, cell_type: CellType) -> Self {
        Self {
            row,
            col,
            value,
            cell_type,
            style: CellStyle::default(),
            formula: None,
        }
    }
}

/// Fila en una hoja
#[derive(Debug, Clone)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub row_number: usize,
}

impl Row {
    pub fn new(cells: Vec<Cell>, row_number: usize) -> Self {
        Self { cells, row_number }
    }
    
    /// Obtener celda por índice de columna
    pub fn get_cell(&self, col_index: usize) -> Option<&Cell> {
        self.cells.get(col_index)
    }
    
    /// Obtener celda mutante por índice
    pub fn get_cell_mut(&mut self, col_index: usize) -> Option<&mut Cell> {
        self.cells.get_mut(col_index)
    }
}

/// Hoja de cálculo
#[derive(Debug, Clone)]
pub struct ExcelSheet {
    pub name: SheetName,
    pub rows: Vec<Row>,
    pub columns: Vec<Column>,
    pub is_hidden: bool,
    pub created_at: std::time::Instant,
    pub last_modified: std::time::Instant,
}

impl ExcelSheet {
    pub fn new(name: SheetName) -> Self {
        Self {
            name,
            rows: Vec::new(),
            columns: Vec::new(),
            is_hidden: false,
            created_at: std::time::Instant::now(),
            last_modified: std::time::Instant::now(),
        }
    }
    
    /// Añadir fila
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
        self.last_modified = std::time::Instant::now();
    }
    
    /// Obtener fila por número
    pub fn get_row(&self, row_number: usize) -> Option<&Row> {
        self.rows.get(row_number)
    }
    
    /// Obtener fila mutante por número
    pub fn get_row_mut(&mut self, row_number: usize) -> Option<&mut Row> {
        self.rows.get_mut(row_number)
    }
    
    /// Obtener número total de filas
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
    
    /// Obtener número total de columnas
    pub fn column_count(&self) -> usize {
        if self.rows.is_empty() {
            return 0;
        }
        self.rows[0].cells.len()
    }
    
    /// Obtener celda por fila y columna
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.rows.get(row)?.get_cell(col)
    }
    
    /// Obtener celda mutante por fila y columna
    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.rows.get_mut(row)?.get_cell_mut(col)
    }
    
    /// Obtener valor de celda
    pub fn get_value(&self, row: usize, col: usize) -> Option<&CellValue> {
        self.get_cell(row, col).map(|c| &c.value)
    }
    
    /// Añadir columna
    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
        self.last_modified = std::time::Instant::now();
    }
    
    /// Obtener columna por nombre
    pub fn get_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name == name)
    }
    
    /// Obtener columna mutante por nombre
    pub fn get_column_mut(&mut self, name: &str) -> Option<&mut Column> {
        self.columns.iter_mut().find(|c| c.name == name)
    }
    
    /// Obtener valor de columna por fila
    pub fn get_column_value(&self, row: usize, column_name: &str) -> Option<&CellValue> {
        if let Some(column) = self.get_column(column_name) {
            if let Some(row) = self.get_row(row) {
                return row.cells.get(column.index).map(|c| &c.value);
            }
        }
        None
    }
}

/// Columna en una hoja
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub index: usize,
    pub data_type: DataType,
    pub is_hidden: bool,
}

impl Column {
    pub fn new(name: &str, index: usize) -> Self {
        Self {
            name: name.to_string(),
            index,
            data_type: DataType::Unknown,
            is_hidden: false,
        }
    }
}

/// Tipo de dato en columna
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Unknown,
    String,
    Number,
    Boolean,
    Date,
}

/// Libro de trabajo de Excel
#[derive(Debug, Default)]
pub struct Workbook {
    pub sheets: Vec<ExcelSheet>,
    pub active_sheet: Option<SheetName>,
    pub created_at: std::time::Instant,
    pub last_modified: std::time::Instant,
}

impl Workbook {
    pub fn new() -> Self {
        Self {
            sheets: Vec::new(),
            active_sheet: None,
            created_at: std::time::Instant::now(),
            last_modified: std::time::Instant::now(),
        }
    }
    
    /// Añadir hoja
    pub fn add_sheet(&mut self, sheet: ExcelSheet) {
        self.sheets.push(sheet);
        self.last_modified = std::time::Instant::now();
    }
    
    /// Obtener hoja por nombre
    pub fn get_sheet(&self, name: &str) -> Option<&ExcelSheet> {
        self.sheets.iter().find(|s| s.name.0 == name)
    }
    
    /// Obtener hoja mutante por nombre
    pub fn get_sheet_mut(&mut self, name: &str) -> Option<&mut ExcelSheet> {
        self.sheets.iter_mut().find(|s| s.name.0 == name)
    }
    
    /// Obtener hoja actual
    pub fn get_active_sheet(&self) -> Option<&ExcelSheet> {
        self.active_sheet.as_ref().and_then(|name| {
            self.sheets.iter().find(|s| s.name.0 == name.0)
        })
    }
    
    /// Obtener hoja mutante actual
    pub fn get_active_sheet_mut(&mut self) -> Option<&mut ExcelSheet> {
        self.active_sheet.as_ref().and_then(|name| {
            self.sheets.iter_mut().find(|s| s.name.0 == name.0)
        })
    }
    
    /// Setear hoja activa
    pub fn set_active_sheet(&mut self, name: &str) {
        self.active_sheet = Some(SheetName(name.to_string()));
        self.last_modified = std::time::Instant::now();
    }
    
    /// Obtener número total de hojas
    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }
    
    /// Obtener nombre de todas las hojas
    pub fn sheet_names(&self) -> Vec<String> {
        self.sheets.iter().map(|s| s.name.0.clone()).collect()
    }
    
    /// Obtener número total de filas en todas las hojas
    pub fn total_rows(&self) -> usize {
        self.sheets.iter().map(|s| s.row_count()).sum()
    }
    
    /// Obtener número total de columnas en todas las hojas
    pub fn total_columns(&self) -> usize {
        self.sheets.iter().map(|s| s.column_count()).max().unwrap_or(0)
    }
    
    /// Importar Bitacora desde Excel
    pub fn import_bitacora(
        &mut self,
        path: &str,
    ) -> Result<Vec<crate::bitacora_manager::BitacoraEntry>, String> {
        // Verificar archivo existe
        if !Path::new(path).exists() {
            return Err(format!("Archivo no encontrado: {}", path));
        }
        
        // Crear workbook desde archivo (simulado)
        // En producción usar librería real de Excel
        let mut entries = Vec::new();
        
        // Simular lectura de archivo Excel
        // En realidad necesitaríamos una librería como calamine para leer .xlsx
        println!("[EXCEL] Importando Bitacora desde: {}", path);
        
        // Devolver entries vacíos (en producción leer del archivo)
        Ok(entries)
    }
}
