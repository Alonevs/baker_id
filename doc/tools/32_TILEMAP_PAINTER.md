# 🖌️ TileMap Painter 32

**Estado:** ⏳ Pendiente | **Prioridad:** 🟡 Media  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Seleccionar una celda del Atlas/Tilesets y "pintar" mapas isométricos u ortogonales directamente en el Viewport con el ratón. Soporta clic y drag para pintar áreas.

### 1.2 Problemas que resuelve
- Permite pintar mapas visualmente
- Elimina edición manual de archivos
- Facilita creación rápida de niveles
- Soporta ambos grid types (iso/ortho)

### 1.3 Usuarios objetivo
- Diseñadores de niveles (usan directamente)
- Artistas (usan para crear tilemaps)
- QA testers (usan para probar niveles)

### 1.4 Requisitos de entrada
- Tileset seleccionado
- Grid type (isométrico u ortogonal)
- Posición en Viewport

### 1.5 Requisitos de salida
- TileMap actualizado en escena
- ID del tile escrito en componente
- Preview en Viewport

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Tileset       │───▶│  Brush Tool     │───▶│  TileMap        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
   [Selected Tile]        [Viewport Click]      [Updated Scene]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| BrushTool | Herramienta pincel | brush_tool.rs | ❌ |
| GridCalculator | Calcular celda | grid_calculator.rs | ❌ |
| TileMapWriter | Escribir tiles | tilemap_writer.rs | ❌ |

### 2.3 Flujo de datos
1. Input: Clic/drag en Viewport + tile seleccionado
2. Process: Calcular celda, escribir ID
3. Output: TileMap actualizado

### 2.4 Dependencias

**Depende de:**
- `forge-scene::TileMap` - Componente TileMap
- `forge-scene::Collider` - Colisiones

**Usado por:**
- `main.rs` - Integración en Toolbar
- `tilemap_system::TileMap` - Sistema de mapas

### 2.5 Interfaz pública (API)

```rust
pub struct BrushTool {
    pub tile_id: u32,
    pub grid_type: GridType,
}

impl BrushTool {
    pub fn paint(&self, viewport_pos: Vec2) -> Result<()> { ... }
    pub fn erase(&self, viewport_pos: Vec2) -> Result<()> { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
// TODO: Implementar
// pub struct BrushTool { ... }
```

### 3.2 Archivos creados
| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| brush_tool.rs | 0 | Herramienta pincel | ❌ Pendiente |

### 3.3 Funcionalidades implementadas
- [ ] Herramienta pincel
- [ ] Clic para pintar
- [ ] Drag para área
- [ ] Borrar (Ctrl)

### 3.4 Funcionalidades pendientes (TO-DO)
- [ ] Cálculo celda isométrica
- [ ] Cálculo celda ortogonal
- [ ] Integración con TileMap
- [ ] Borrar tiles

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
// TODO: Implementar
#[test]
fn test_calculate_tile() { ... }
```

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
// TODO: Ejemplo
let mut brush = BrushTool::new();
brush.paint(viewport_pos);
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Pendiente ⏳)
- [ ] Herramienta pincel
- [ ] Clic/drag
- [ ] Grid calculation

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] Integración TileMap
- [ ] Borrar tiles
- [ ] Preview en tiempo real

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]