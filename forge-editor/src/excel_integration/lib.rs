pub use excel_reader::{
    Workbook,
    ExcelSheet,
    Cell,
    Row,
    CellValue,
    CellType,
    CellStyle,
    CellAlignHorizontal,
    CellAlignVertical,
    CellRange,
    SheetName,
    DataType,
    Column,
    DateRange,
};

pub use excel_writer::{
    ExcelWriter,
    PendingChange,
    ChangeType,
};

pub use sheet_manager::{
    SheetManager,
    DataValidator,
    ValidationRule,
    ValidationError,
};

/// Integración de Bitacora con Excel
pub mod bitacora_excel_integration {
    use super::*;
    use crate::bitacora_manager::BitacoraManager;

    /// Exportar Bitacora a archivo Excel
    pub fn export_bitacora_to_excel(
        manager: &BitacoraManager,
        filename: &str,
    ) -> Result<(), String> {
        let mut writer = ExcelWriter::new();
        writer.export_bitacora(manager, filename)
    }

    /// Importar Bitacora desde archivo Excel
    pub fn import_bitacora_from_excel(
        path: &str,
    ) -> Result<Vec<crate::bitacora_manager::BitacoraEntry>, String> {
        let mut workbook = Workbook::new();
        workbook.import_bitacora(path)
    }
}
