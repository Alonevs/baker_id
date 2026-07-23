# 📸 3D Sprite Baker 30

**Estado:** ⏳ Pendiente | **Prioridad:** 🟡 Media  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Generador de hojas de sprites de 360 grados a partir de modelos 3D (.gltf/.obj). Renderiza múltiples ángulos con offset de píxeles y exporta a formato atlas con validación de paleta 256 colores.

### 1.2 Problemas que resuelve
- Automatiza generación de sprites 360°
- Elimina renderizado manual en editor
- Asegura consistencia en sprites
- Valida restricción cromática (256 colores)

### 1.3 Usuarios objetivo
- Artistas 3D (usan directamente)
- Diseñadores (benefician con sprites listos)
- Programadores (benefician con assets consistentes)

### 1.4 Requisitos de entrada
- Modelo 3D (.gltf/.obj)
- Configuración de resolución
- Configuración de ángulos

### 1.5 Requisitos de salida
- Hoja de sprites en `/assets/sprites/`
- SpriteSheet serializado
- Registro en AssetManager

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   3D Model      │───▶│  Render Engine  │───▶│  Sprite Sheet   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
   [Model File]        [360° Render]        [Atlas + Validation]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| SpriteBaker | Gestor principal | sprite_baker.rs | ❌ |
| ModelRenderer | Render modelo 3D | model_renderer.rs | ❌ |
| PaletteValidator | Validar 256 colores | palette_validator.rs | ❌ |
| AtlasExporter | Export atlas | atlas_exporter.rs | ❌ |

### 2.3 Flujo de datos
1. Input: Modelo 3D + configuración
2. Process: Render 360°, aplicar offset, validar paleta
3. Output: SpriteSheet en `/assets/sprites/`

### 2.4 Dependencias

**Depende de:**
- `rendering` - Renderizador 3D
- `image` - Procesamiento de imágenes
- `palette` - Validación de paletas

**Usado por:**
- `main.rs` - Integración en Asset Browser
- `asset_manager::AssetManager` - Registro de sprites

### 2.5 Interfaz pública (API)

```rust
pub struct SpriteBaker {
    pub resolution: Vec2,
    pub angles: Vec<f32>,
}

impl SpriteBaker {
    pub fn bake(&mut self, model_path: &str) -> Result<SpriteSheet> { ... }
    pub fn validate_palette(&self, sprite: &Sprite) -> Result<()> { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
// TODO: Implementar
// pub struct SpriteBaker { ... }
```

### 3.2 Archivos creados
| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| sprite_baker.rs | 0 | Gestor principal | ❌ Pendiente |

### 3.3 Funcionalidades implementadas
- [ ] **Render 360°** - Renderizar múltiples ángulos
- [ ] **Offset de píxeles** - Aplicar offset
- [ ] **Validación paleta** - 256 colores máx
- [ ] **Export atlas** - Guardar en `/assets/sprites/`

### 3.4 Funcionalidades pendientes (TO-DO)
- [ ] Integración con renderizador 3D
- [ ] UI en "3D Baker Panel"
- [ ] Registro en AssetManager
- [ ] Filtro cromático automático

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
// TODO: Implementar
#[test]
fn test_bake_angle() { ... }
```

### 4.2 Test de Integración
```rust
// TODO: Implementar
#[test]
fn test_export_sprite_sheet() { ... }
```

### 4.4 Estado de tests
| Test Suite | Passing | Total | Rate |
|------------|---------|-------|------|
| Unit Tests | 0/0 | N/A | ⏳ |
| Integration | 0/0 | N/A | ⏳ |
| **TOTAL** | **0/0** | **N/A** | **⏳** |

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
// TODO: Ejemplo
let mut baker = SpriteBaker::new();
baker.bake("/models/character.glb");
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |
| Tests passing | 0/0 | 100% | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Pendiente ⏳)
- [ ] Renderizador 3D
- [ ] Generación 360°
- [ ] Export atlas

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] UI Panel
- [ ] Validación paleta
- [ ] Registro AssetManager

---

## ⚠️ HERRAMIENTAS INTEGRADAS (Añadidas desde catálogo)

### Sprite Baker
- **Atlas generation** - Generar hojas de sprites
- **Sprite sheet packing** - Empaquetado eficiente
- **Metadata extraction** - Extraer metadatos de sprites
- **File:** `forge-editor/src/sprite_baker/`

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]