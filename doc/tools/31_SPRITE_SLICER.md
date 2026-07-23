# ✂️ Sprite & Sheet Slicer 31

**Estado:** ⏳ Pendiente | **Prioridad:** 🟡 Media  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Trocear imágenes PNG en celdas, definir tilesets, ajustar márgenes y validar paleta de color (máx 256 colores). Aplica filtro cromático si excede límite.

### 1.2 Problemas que resuelve
- Automatiza creación de tilesets
- Valida restricción cromática (256 colores)
- Facilita creación de atlas
- Elimina trabajo manual de troceado

### 1.3 Usuarios objetivo
- Artistas 2D (usan directamente)
- Diseñadores (benefician con tilesets listos)
- QA testers (benefician con validación automática)

### 1.4 Requisitos de entrada
- Imagen PNG
- Configuración de grid
- Configuración de márgenes

### 1.5 Requisitos de salida
- Tileset serializado (.tileset)
- Coordenadas UV
- Metadata de tiles

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   PNG Image     │───▶│  Slice Engine   │───▶│  Tileset + UV   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
   [Source Image]        [Grid + Margins]      [Tile + Metadata]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| SliceEngine | Trocear imagen | slice_engine.rs | ❌ |
| TilesetDef | Definir tileset | tileset_def.rs | ❌ |
| PaletteValidator | Validar paleta | palette_validator.rs | ❌ |
| UVExporter | Export UV coords | uv_exporter.rs | ❌ |

### 2.3 Flujo de datos
1. Input: PNG + configuración de grid
2. Process: Trocear, validar paleta, calcular UV
3. Output: Tileset con metadata

### 2.4 Dependencias

**Depende de:**
- `image` - Procesamiento PNG
- `palette` - Validación de paletas

**Usado por:**
- `main.rs` - Integración en Asset Browser
- `tilemap_system::TileMap` - Sistema de mapas

### 2.5 Interfaz pública (API)

```rust
pub struct SliceEngine {
    pub grid_size: Vec2,
    pub margins: Vec2,
}

impl SliceEngine {
    pub fn slice(&self, image: &Image) -> Result<Vec<Tile>> { ... }
    pub fn validate_palette(&self, image: &Image) -> Result<PaletteInfo> { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
// TODO: Implementar
// pub struct SliceEngine { ... }
```

### 3.2 Archivos creados
| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| slice_engine.rs | 0 | Trocear imagen | ❌ Pendiente |

### 3.3 Funcionalidades implementadas
- [ ] Trocear imagen en celdas
- [ ] Definir tileset
- [ ] Ajustar márgenes
- [ ] Validar paleta 256 colores

### 3.4 Funcionalidades pendientes (TO-DO)
- [ ] UI de troceado
- [ ] Filtro cromático automático
- [ ] Export .tileset
- [ ] Export UV coords

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
// TODO: Implementar
#[test]
fn test_slice_image() { ... }
```

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
// TODO: Ejemplo
let mut engine = SliceEngine::new();
engine.slice("tileset.png");
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Pendiente ⏳)
- [ ] Troceado básico
- [ ] Validación paleta
- [ ] Export tileset

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] UI Editor
- [ ] Filtro cromático
- [ ] Export UV

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]