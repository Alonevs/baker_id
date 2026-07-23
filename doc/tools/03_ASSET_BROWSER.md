## 📚 REFERENCIAS

### 11.1 Archivos Relacionados

| Archivo | Líneas | Función |
|---------|--------|---------|
| `src/ui/asset_browser.rs` | 694 | Asset Browser completo |
| `src/ui/asset_preview.rs` | - | Preview de assets |
| `src/ui/tab_viewer.rs` | - | Tab AssetBrowser |
| `src/import_manager.rs` | 174 | Importación de assets |
| `src/main.rs` | - | ForgeEditorApp |

### 11.2 Dependencias Externas

| Crate | Versión | Función |
|-------|---------|---------|
| `std::fs` | - | Lectura de directorios |
| `std::path` | - | Manipulación de paths |
| `std::collections::HashMap` | - | Tree de assets |
| `pollster::FutureExt` | - | `block_on()` para rfd |
| `rfd::AsyncFileDialog` | - | Selector de directorio |
| `egui` | - | UI rendering |

---

**Última actualización:** 2026-07-23  
**AI:** [AI: opencode]
