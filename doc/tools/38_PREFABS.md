# 🗂️ Presets & Prefabs 38

**Estado:** ⏳ Pendiente | **Prioridad:** 🟡 Media  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Proveer conjuntos empaquetados de componentes preconfigurados (Presets) y guardar entidades terminadas como Plantillas (Prefabs .prefab JSON). Permite instanciación ágil y duplicación.

### 1.2 Problemas que resuelve
- Reutilización de configuraciones
- Instanciación rápida de entidades
- Consistencia entre entidades
- Aceleración de desarrollo

### 1.3 Usuarios objetivo
- Diseñadores (usan para prefabs)
- Programadores (usan para presets)
- QA testers (usan para pruebas rápidas)

### 1.4 Requisitos de entrada
- Entidad configurada
- Preset definido
- Ruta en `assets/prefabs/`

### 1.5 Requisitos de salida
- Archivo .prefab JSON
- Preset aplicado
- Entidad instanciada

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Entity        │───▶│  Prefab System  │───▶│  .prefab JSON   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
   [Configured]        [Serialize]            [Load & Duplicate]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| PrefabManager | Gestor prefabs | prefab_manager.rs | ❌ |
| PresetLibrary | Librería presets | preset_library.rs | ❌ |
| PrefabSerializer | Serialización JSON | prefab_serializer.rs | ❌ |

### 2.3 Flujo de datos
1. Input: Entidad configurada
2. Process: Serializar a JSON .prefab
3. Output: Prefab en `assets/prefabs/`

### 2.4 Dependencias

**Depende de:**
- `serde_json` - Serialización
- `forge-scene::Scene` - Estructura escena

**Usado por:**
- `main.rs` - Integración en Inspector
- `asset_manager::AssetManager` - Cargar prefabs

### 2.5 Interfaz pública (API)

```rust
pub struct PrefabManager {
    pub presets: HashMap<String, Preset>,
}

impl PrefabManager {
    pub fn save_as_prefab(&self, entity: &Entity) -> Result<PathBuf> { ... }
    pub fn instantiate_prefab(&self, path: &str) -> Result<Entity> { ... }
    pub fn apply_preset(&self, entity: &Entity, preset_name: &str) -> Result<()> { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
// TODO: Implementar
// pub struct PrefabManager { ... }
```

### 3.2 Archivos creados
| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| prefab_manager.rs | 0 | Gestor | ❌ Pendiente |

### 3.3 Funcionalidades implementadas
- [ ] Serializar a .prefab JSON
- [ ] Guardar en `assets/prefabs/`
- [ ] Instanciar prefab
- [ ] Aplicar presets

### 3.4 Funcionalidades pendientes (TO-DO)
- [ ] Botón "Guardar como Plantilla"
- [ ] Arrastrar prefab al Viewport
- [ ] Menú de presets
- [ ] Duplicar prefab

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
// TODO: Implementar
#[test]
fn test_save_prefab() { ... }
```

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
// TODO: Ejemplo
let mut manager = PrefabManager::new();
manager.save_as_prefab(entity);
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Pendiente ⏳)
- [ ] Serialización JSON
- [ ] Guardar prefabs
- [ ] Instanciar

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] Botón en Inspector
- [ ] Drag & drop
- [ ] Menú presets

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]