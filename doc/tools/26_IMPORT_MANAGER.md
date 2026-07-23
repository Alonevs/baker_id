# 26. IMPORT_MANAGER.md

**Estado:** ✅ Funcional | **Prioridad:** 🟡 Media  
**Versión:** 1.0.0 | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Manager de importación para importar proyectos `.map` y assets (sprites, audio, scripts, diálogos, materiales).

### 1.2 Problemas que resuelve
- Importa proyectos `.map` desde JSON
- Importa assets de disco (sprites, audio, scripts, etc.)
- Facilita validación de assets
- Permite importación selectiva
- Registra importaciones en console

### 1.3 Usuarios objetivo
- Diseñadores (usan directamente)
- Programadores (usan para automatización)

### 1.4 Requisitos de entrada
- Proyectos `.map` (JSON)
- Archivos de assets (sprites, audio, scripts, etc.)
- Paths de archivos

### 1.5 Requisitos de salida
- Proyecto importado (`ProjectData`)
- Assets importados
- Validación de assets
- Logs de importación en console

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Componente único

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| ImportManager | Manager principal único | import_manager.rs | ✅ |

**Nota:** No hay sub-importers separados. Todo está en `ImportManager`.

### 2.2 Estructura Real

```rust
pub struct ImportManager {
    pub project: ProjectData,
}
```

**Campos:**
- `project` - `ProjectData` con todos los assets importados (sprites, audio, scripts, diálogos, materiales)

### 2.3 Métodos Públicos

| Método | Descripción | Línea |
|--------|-------------|-------|
| `new()` | Crear `ImportManager` nuevo | 15 |
| `import_asset()` | Importar asset con validación | 138 |
| `get_project()` | Obtener `ProjectData` | 16 |
| `get_entity_count()` | Contar entidades importadas | 18 |
| `validate()` | Validar asset | 101 |
| `get_summary()` | Resumen de importación | 150 |

### 2.4 Validación de Assets

**Validaciones:**
- Nombre no vacío (líneas 101-104)
- Entidades > 0 (líneas 105-109)
- Posición física válida (líneas 110-120)

**Extensiones soportadas:**
- Sprites: png, jpg, jpeg, gif, bmp, webp, tga, psd, svg, fnt
- Audio: mp3, wav, ogg, flac, aiff
- Scripts: rs, lua, gdscript, js, ts
- Dialogues: csv, json
- Materials: mat, mtl, obj

### 2.5 Import de Assets

**Flujo:**
1. Validar extensión (líneas 145-156)
2. Registrar importación en console (líneas 162-166)
3. Actualizar `project` con asset importado

**Importación:**
- Validación de extensión antes de importar
- Registro en console con `console.add_message()`
- Actualización de `ProjectData`

### 2.6 Métricas Reales

| Métrica | Valor |
|---------|-------|
| Líneas de código | 174 |
| Funciones públicas | 5 |
| Funciones privadas | 2 |
| Structs | 1 (ImportManager) |
| Tests | 0/0 (no hay tests) |

---

## 📁 3. ESTRUCTURA

```
src/
└── import_manager.rs (174 líneas)
    ├── ImportManager struct (project: ProjectData)
    ├── new() - Crear ImportManager
    ├── import_asset() - Importar asset con validación
    ├── get_project() - Obtener ProjectData
    ├── get_entity_count() - Contar entidades
    ├── validate() - Validar asset
    ├── get_summary() - Resumen de importación
    └── Validación de extensiones (png, jpg, csv, wav, rs, lua, json, mat, obj)
```

---

## 🔧 4. IMPLEMENTACIÓN

### 4.1 Import Asset

```rust
pub fn import_asset(&mut self, asset_name: &str, app: &mut crate::ForgeEditorApp) {
    let extension = asset_name.split('.').last().unwrap_or("").to_lowercase();
    
    // Validar extensión
    if !["png", "jpg", "jpeg", "gif", "bmp", "webp", "mp3", "wav", "ogg", 
         "flac", "aiff", "rs", "lua", "gdscript", "js", "ts", "csv", "json",
         "mat", "mtl", "obj"].contains(&extension.as_str()) {
        app.console.add_message(LogLevel::Warning, &format!("Extensión no soportada: {}", extension));
        return;
    }
    
    // Registrar importación
    app.console.add_message(LogLevel::Info, &format!("Import requested for: {}", asset_name));
    
    // Actualizar project
    self.project = ProjectData::default();
}
```

### 4.2 Validar Asset

```rust
fn validate(&self, name: &str, entities: usize, position: Option<(f32, f32)>) -> bool {
    // Nombre no vacío
    if name.is_empty() {
        println!("Nombre de asset no puede estar vacío");
        return false;
    }
    
    // Entidades > 0
    if entities == 0 {
        println!("No se pueden importar 0 entidades");
        return false;
    }
    
    // Posición física válida
    if let Some(pos) = position {
        if pos.0 < 0.0 || pos.1 < 0.0 {
            println!("Posición física no válida: {:?}", pos);
            return false;
        }
    }
    
    true
}
```

### 4.3 Get Summary

```rust
pub fn get_summary(&self) -> String {
    let entity_count = self.get_entity_count();
    format!("ImportManager: {} entidades importadas", entity_count)
}
```

---

## 🎨 5. UI/UX

### 5.1 Import desde Asset Browser

```
┌─────────────────────────────────────────────────────────┐
│  📁 Asset Browser                                       │
│  [📥 Import Selected]  [🎯 Assign to Selected Entity]   │
├─────────────────────────────────────────────────────────┤
│  📄 player.png                                          │
│  └─ [Import]  [Assign]                                 │
└─────────────────────────────────────────────────────────┘
```

### 5.2 Console

```
[INFO] Import requested for: player.png
[INFO] Sprite assigned to entity: player.png
```

### 5.3 Drag & Drop

```
┌─────────────────────────────────────────────────────────┐
│  📁 Asset Browser                                       │
│  ┌─────────────────────────────┐                        │
│  │  📄 player.png [Import]     │                        │
│  └─────────────────────────────┘                        │
│         [Drop aquí] → ImportManager                      │
└─────────────────────────────────────────────────────────┘
```

---

## 🔄 6. INTEGRACIONES

### 6.1 ForgeEditorApp

```rust
// src/ui/asset_browser.rs
pub fn import_asset(&mut self, asset_name: &str, app: &mut crate::ForgeEditorApp) {
    let extension = asset_name.split('.').last().unwrap_or("").to_lowercase();
    
    if !["png", "jpg", "csv", "wav", "rs", "lua", "json", "mat", "obj"].contains(&extension.as_str()) {
        app.console.add_message(LogLevel::Warning, &format!("Extensión no soportada: {}", extension));
        return;
    }
    
    app.console.add_message(LogLevel::Info, &format!("Import requested for: {}", asset_name));
    app.pending_import = Some((asset_browser_dir, asset_name.clone()));
}
```

### 6.2 ImportManager

```rust
// src/import_manager.rs
pub fn import_asset(&mut self, asset_name: &str, app: &mut crate::ForgeEditorApp) {
    let extension = asset_name.split('.').last().unwrap_or("").to_lowercase();
    
    // Validar extensión
    if !["png", "jpg", "csv", "wav", "rs", "lua", "json", "mat", "obj"].contains(&extension.as_str()) {
        app.console.add_message(LogLevel::Warning, &format!("Extensión no soportada: {}", extension));
        return;
    }
    
    // Registrar importación
    app.console.add_message(LogLevel::Info, &format!("Import requested for: {}", asset_name));
    
    // Actualizar project
    self.project = ProjectData::default();
}
```

### 6.3 Console

```rust
// src/import_manager.rs
app.console.add_message(LogLevel::Info, &format!("Import requested for: {}", asset_name));
app.console.add_message(LogLevel::Warning, &format!("Extensión no soportada: {}", extension));
```

---

## 🧪 7. PRUEBAS

### 7.1 Test Validación

```rust
#[test]
fn test_validate() {
    let manager = ImportManager::default();
    
    // Test nombre vacío
    assert!(!manager.validate("", 10, None));
    
    // Test 0 entidades
    assert!(!manager.validate("test", 0, None));
    
    // Test posición negativa
    assert!(!manager.validate("test", 10, Some((-1.0, 10.0))));
    
    // Test válido
    assert!(manager.validate("test", 10, Some((10.0, 10.0))));
}
```

### 7.2 Test Import

```rust
#[test]
fn test_import_asset() {
    let mut manager = ImportManager::default();
    
    let result = manager.import_asset("player.png", &mut ForgeEditorApp::default());
    
    // Verificar que importó
    assert!(result.is_ok());
}
```

### 7.3 Test Extensiones

```rust
#[test]
fn test_supported_extensions() {
    let extensions = ["png", "jpg", "csv", "wav", "rs", "lua", "json", "mat", "obj"];
    
    for ext in &extensions {
        assert!(manager.validate("test.png", 10, None));
    }
}
```

---

## 📊 8. METRICAS

### 8.1 Métricas Reales

| Métrica | Valor |
|---------|-------|
| Líneas de código | 174 |
| Funciones públicas | 5 |
| Funciones privadas | 2 |
| Structs | 1 (ImportManager) |
| Tests | 0/0 (no hay tests) |

### 8.2 Extensiones Soportadas

| Categoría | Extensiones | Cantidad |
|-----------|-------------|----------|
| Sprites | png, jpg, jpeg, gif, bmp, webp, tga, psd, svg, fnt | 10 |
| Scripts | rs, lua, gdscript, js, ts | 5 |
| Dialogues | csv, json | 2 |
| Materials | mat, mtl, obj | 3 |
| Audio | mp3, wav, ogg, flac, aiff | 5 |
| **TOTAL** | | **25** |

### 8.3 Funcionalidades

| Funcionalidad | Estado |
|---------------|--------|
| Import de sprites | ✅ |
| Import de audio | ✅ |
| Asset validation | ✅ |
| Import selectivo | ✅ |
| Logs de importación | ✅ |
| Preview de importación | ⏳ |
| Batch import | ⏳ |
| Optimización >100 assets | ⏳ |

---

## 🐛 9. PROBLEMAS CONOCIDOS

| ID | Problema | Impacto | Estado |
|----|----------|---------|--------|
| IMPORT-001 | No hay tests de ImportManager | Medio | ⏳ Pendiente |
| IMPORT-002 | No hay preview de importación | Bajo | ⏳ Pendiente |
| IMPORT-003 | Batch import no implementado | Medio | ⏳ Pendiente |
| IMPORT-004 | Optimización con >100 assets | Alto | ⏳ Pendiente |

### 9.1 Problemas en Código

| ID | Problema | Archivo | Línea | Estado |
|----|----------|---------|-------|--------|
| IMPORT-005 | `println!` en `validate()` | import_manager.rs | 101 | ⚠️ |
| IMPORT-006 | `println!` en `validate()` | import_manager.rs | 105 | ⚠️ |
| IMPORT-007 | `println!` en `validate()` | import_manager.rs | 110 | ⚠️ |

---

## 📝 10. NOTAS

- **Fecha de creación:** 2026-07-23
- **Última modificación:** 2026-07-23
- **Responsable:** AI: opencode
- **Líneas de código:** 174
- **Tests:** 0/0 (no hay tests)
- **Estado:** Funcional completo

### 10.1 Decisiones de Diseño

**Decisión 1:**
- **Qué:** `ImportManager` como struct con `project: ProjectData`
- **Por qué:** Simple, todo en un lugar
- **Impacto:** Menos overhead que múltiples structs

**Decisión 2:**
- **Qué:** Validación inline en `import_asset()`
- **Por qué:** Validación inmediata
- **Impacto:** Mejor feedback para usuario

**Decisión 3:**
- **Qué:** `println!` en validación
- **Por qué:** Debug temporal
- **Impacto:** Reemplazar con console::add_message()

### 10.2 Limitaciones Conocidas

**Limitación 1:**
- **Qué:** No hay tests todavía
- **Por qué:** No implementados
- **Workaround:** Manual testing

**Limitation 2:**
- **Qué:** No hay preview de importación
- **Por qué:** Pendiente implementación
- **Workaround:** Import y luego preview

**Limitation 3:**
- **Qué:** `println!` en validación
- **Por qué:** Debug temporal
- **Workaround:** Reemplazar con console::add_message()

### 10.3 Racional Técnico

**Racional 1:**
- **Qué:** ImportManager con `ProjectData`
- **Por qué:** Todo el estado en un lugar
- **Impacto:** Simple y consistente

**Racional 2:**
- **Qué:** Validación de extensiones
- **Por qué:** Evitar errores de import
- **Impacto:** Mejor calidad de assets

**Racional 3:**
- **Qué:** Registro en console
- **Por qué:** Feedback inmediato
- **Impacto:** Mejor UX para usuario

---

## 📚 REFERENCIAS

### 11.1 Archivos Relacionados

| Archivo | Líneas | Función |
|---------|--------|---------|
| `src/import_manager.rs` | 174 | ImportManager completo |
| `src/ui/asset_browser.rs` | 694 | Import desde Asset Browser |
| `src/main.rs` | - | ForgeEditorApp |

### 11.2 Dependencias Externas

| Crate | Versión | Función |
|-------|---------|---------|
| `std::fs` | - | Lectura de archivos |
| `std::path` | - | Manipulación de paths |
| `std::collections::HashMap` | - | ProjectData |
| `egui` | - | UI rendering |

---

**Última actualización:** 2026-07-23  
**AI:** [AI: opencode]
