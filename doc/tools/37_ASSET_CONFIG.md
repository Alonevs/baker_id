# 📦 Configuración de Assets 37

**Estado:** ⏳ Pendiente | **Prioridad:** 🟡 Media  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Definir y persistir propiedades globales de importación por cada asset (imágenes, sonidos). Configura escala, filtrado visual (Nearest/Linear), volumen, bucles, y guarda en archivos .meta.

### 1.2 Problemas que resuelve
- Configuración per-asset
- Persistencia de import settings
- Consistencia en assets
- Reutilización de configuraciones

### 1.3 Usuarios objetivo
- Artistas (usan para configurar assets)
- Diseñadores (usan para import settings)
- Programadores (usan para leer .meta)

### 1.4 Requisitos de entrada
- Asset (imagen, sonido)
- Configuración de import
- Ruta del asset

### 1.5 Requisitos de salida
- Archivo .meta junto al asset
- Configuración aplicada al asset
- Importación con settings guardados

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Asset Browser │───▶│  Asset Config   │───▶│  .meta File     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
   [Select Asset]        [Configure]          [Save Next To]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| AssetConfig | Configuración | asset_config.rs | ❌ |
| MetaFileWriter | Escribir .meta | meta_writer.rs | ❌ |
| ImportSettings | Settings import | import_settings.rs | ❌ |

### 2.3 Flujo de datos
1. Input: Asset seleccionado
2. Process: Configurar, guardar .meta
3. Output: Asset con settings persistidos

### 2.4 Dependencias

**Depende de:**
- `serde_json` - Serialización
- `image` - Configuración imágenes
- `audio` - Configuración audio

**Usado por:**
- `main.rs` - Integración en Inspector
- `asset_manager::AssetManager` - Importación

### 2.5 Interfaz pública (API)

```rust
pub struct AssetConfig {
    pub scale: f32,
    pub filter_mode: FilterMode,
    pub volume: f32,
    pub loop: bool,
}

impl AssetConfig {
    pub fn save_meta(&self, asset_path: &str) -> Result<()> { ... }
    pub fn load_meta(&self, asset_path: &str) -> Result<()> { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
// TODO: Implementar
// pub struct AssetConfig { ... }
```

### 3.2 Archivos creados
| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| asset_config.rs | 0 | Configuración | ❌ Pendiente |

### 3.3 Funcionalidades implementadas
- [ ] Configurar escala
- [ ] Configurar filtrado (Nearest/Linear)
- [ ] Configurar volumen/bucles
- [ ] Guardar .meta

### 3.4 Funcionalidades pendientes (TO-DO)
- [ ] Modo Asset en Inspector
- [ ] Leer .meta al arrastrar
- [ ] Persistencia automática
- [ ] UI de configuración

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
// TODO: Implementar
#[test]
fn test_save_meta() { ... }
```

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
// TODO: Ejemplo
let mut config = AssetConfig::new();
config.save_meta("/assets/sprite.png");
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Pendiente ⏳)
- [ ] Configuración
- [ ] Guardar .meta
- [ ] Leer .meta

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] Modo Asset Inspector
- [ ] Persistencia automática
- [ ] UI configuración

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]