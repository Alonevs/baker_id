# 🔄 LiveSync 01

**Estado:** 🔄 Reconstruyendo | **Prioridad:** 🔴 Alta  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Sistema de sincronización en tiempo real entre Editor y Runtime. Permite sincronizar cambios de escena, assets, y estado del juego en tiempo real con pub/sub, delta sync, y resolución de conflictos para colaboración multi-usuario.

### 1.2 Flujo de Bienvenida y Modo Tutor

**Pantalla Modal de Inicio:**
- Selector de ruta: [📁 Nuevo Proyecto] | [📂 Cargar Proyecto]
- Interruptor de género: 4 perfiles (Isométrico, Ortogonal, Sprites Libres, Lienzo Rígido)
- Inicialización del entorno: carga silenciosa según género elegido

**Modo Tutor Interactivo:**
- Ubicación: Pestaña flotante translúcida en esquina superior derecha del Visor Central
- Checklist dinámico que guía los primeros pasos:
  - 🟩 Paso 1 (Datos): Arrastrar primer PNG de fondo o spritesheet al Explorador de Archivos (Abajo Izquierda)
  - 🟨 Paso 2 (Posicionamiento): Seleccionar asset importado y arrastrar al Canvas Central para colocarlo en Capa correspondiente
  - ⬛ Paso 3 (Mecánicas): Usar Pincel de Físicas para trazar líneas de colisión por donde correrá el personaje
- Botón de silencio: [🔕 Ocultar Guía] - desaparece con animación fluida liberando 100% del espacio visual del visor

### 1.3 Problemas que resuelve
- Elimina la necesidad de recargar la escena manualmente
- Permite edición en tiempo real sin perder progreso
- Facilita la colaboración multi-usuario en el mismo proyecto
- Guía a nuevos usuarios con checklist interactivo

### 1.2 Problemas que resuelve
- Elimina la necesidad de recargar la escena manualmente
- Permite edición en tiempo real sin perder progreso
- Facilita la colaboración multi-usuario en el mismo proyecto

### 1.3 Usuarios objetivo
- Diseñadores (usan directamente)
- Programadores (benefician con hot-reload)
- QA testers (benefician con cambios en tiempo real)

### 1.4 Requisitos de entrada
- Scene actualizada en memoria
- Event pub/sub para cambios
- Protocolo de sincronización definido

### 1.5 Requisitos de salida
- Runtime sincronizado con Editor
- Delta sync (solo cambios)
- Estado consistente en todos los clientes

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Editor State  │───▶│  Sync Manager   │───▶│   Runtime State │
└─────────────────┘    └─────────────────┘    └─────────────────┘
      │                      │                      │
      ▼                      ▼                      ▼
  [Scene Change]        [Event Pub/Sub]        [Apply State]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| SyncManager | Gestor principal | live_sync_manager.rs | ❌ |
| EventBus | Pub/Sub para eventos | event_bus.rs | ❌ |
| SyncEvent | Tipos de eventos | sync_event.rs | ❌ |
| DeltaEncoder | Codificar cambios | delta_encoder.rs | ❌ |
| ConflictResolver | Resolver conflictos | conflict_resolver.rs | ❌ |

### 2.3 Flujo de datos
1. Input: Cambios en escena (add/remove/modify entity)
2. Process: Publicar evento, codificar delta, aplicar a subscribers
3. Output: Runtime actualizado con cambios sincronizados

### 2.4 Dependencias

**Depende de:**
- `forge-scene::Scene` - Estructura de escena
- `forge-panel-messaging::EventBus` - Sistema pub/sub
- `serde` - Serialización

**Usado por:**
- `main.rs` - Integración en editor principal
- `runtime::GameLoop` - Runtime del juego
- `asset_manager::AssetManager` - Assets

### 2.5 Interfaz pública (API)

```rust
pub struct SyncManager {
    pub subscribers: HashMap<EntityId, Vec<Subscriber>>,
    pub publishers: Vec<Publisher>,
}

impl SyncManager {
    pub fn new() -> Self { ... }
    pub fn subscribe(&mut self, entity_id: EntityId, callback: Subscriber) { ... }
    pub fn publish(&self, event: SyncEvent) { ... }
    pub fn sync_scene(&mut self, scene: &Scene) { ... }
}

pub enum SyncEvent {
    SceneLoaded(SceneId),
    EntityAdded(EntityId),
    EntityRemoved(EntityId),
    ComponentChanged(EntityId, ComponentId, &Component),
    TransformChanged(EntityId, &Transform),
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado

```rust
// TODO: Implementar estructura básica
// pub struct SyncManager { ... }

// TODO: Implementar eventos
// pub enum SyncEvent { ... }

// TODO: Implementar pub/sub
// pub struct EventBus { ... }
```

### 3.2 Archivos creados

| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| live_sync_manager.rs | 0 | Gestor principal | ❌ Pendiente |
| sync_event.rs | 0 | Tipos de eventos | ❌ Pendiente |
| event_bus.rs | 0 | Pub/Sub | ❌ Pendiente |

### 3.3 Funcionalidades implementadas

- [ ] **Sync Manager** - Gestor principal de sincronización
- [ ] **Event Pub/Sub** - Sistema de eventos
- [ ] **Delta Sync** - Enviar solo cambios
- [ ] **Conflict Resolution** - Resolver conflictos

### 3.4 Funcionalidades pendientes (TO-DO)

- [ ] **Editor ↔ Runtime** - Sincronizar escena con runtime
- [ ] **Assets ↔ Preview** - Sincronizar assets con preview
- [ ] **Multi-user** - Colaboración en tiempo real
- [ ] **Live Reload** - Hot reload de cambios

---

## 🧪 4. TESTS

### 4.1 Test Unitario

```rust
// TODO: Implementar tests
#[test]
fn test_subscribe_publish() { ... }

#[test]
fn test_sync_scene() { ... }
```

### 4.2 Test de Integración

```rust
// TODO: Implementar tests
#[test]
fn test_editor_runtime_sync() { ... }

#[test]
fn test_asset_sync() { ... }
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
// TODO: Ejemplo básico
let mut sync = SyncManager::new();
sync.subscribe(entity_id, callback);
sync.publish(SyncEvent::EntityAdded(entity_id));
```

### 5.2 Ejemplo de uso avanzado

```rust
// TODO: Ejemplo avanzado
let mut sync = SyncManager::new();

// Suscribirse a cambios
sync.subscribe(entity_id, |event| {
    handle_sync_event(event);
});

// Sincronizar escena completa
sync.sync_scene(&scene);
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |
| Funciones públicas | 0 | < 50 | ⏳ |
| Tests passing | 0/0 | 100% | ⏳ |
| Coverage | 0% | > 90% | ⏳ |

---

## 🐛 7. PROBLEMAS CONOCIDOS

| ID | Problema | Impacto | Prioridad | Estado |
|----|----------|---------|-----------|--------|
| BUG-001 | Sistema perdido, necesita reconstrucción | Alto | 🔴 | 🔄 |
| BUG-002 | No hay tests existentes | Medio | 🟡 | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (En progreso 🔄)
- [ ] Sync Manager básico
- [ ] Event Pub/Sub
- [ ] Tipos de eventos
- [ ] Tests básicos

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] Delta sync
- [ ] Editor ↔ Runtime
- [ ] Assets ↔ Preview
- [ ] LiveSync con Bitacora Manager (Feature X)

### 8.3 Fase 3: Avanzado (Pendiente ⏳)
- [ ] Multi-user
- [ ] Conflict resolution
- [ ] Live reload

---

## 📝 9. NOTAS Y DECISIONES

### 9.1 Decisiones de diseño

**Decisión 1:**
- **Qué:** Pub/Sub sobre direct calls
- **Por qué:** Mejor desacoplamiento y escalabilidad
- **Impacto:** Código más limpio pero complejidad en debugging

**Decisión 2:**
- **Qué:** Delta sync (solo cambios) sobre full sync
- **Por qué:** Menos datos, más eficiente
- **Impacto:** Menor latencia y ancho de banda

### 9.2 Limitaciones conocidas

**Limitación 1:**
- **Qué:** Sistema perdido, necesita reconstrucción
- **Por qué:** Código original no existe
- **Workaround:** Implementar desde cero

**Limitación 2:**
- **Qué:** No soporta multi-user todavía
- **Por qué:** Necesita WebSocket server/client
- **Workaround:** Single-user development

**Limitación 3:**
- **Qué:** No hay LiveSync con Bitacora Manager
- **Por qué:** Pendiente integración
- **Workaround:** Actualizar manual
- **Workaround:** BIT-001: No hay LiveSync con CSV (Medio, 🔄)

### 9.3 Racional Técnico

**Racional 1:**
- **Qué:** SyncManager como singleton
- **Por qué:** Un solo punto de sincronización
- **Impacto:** Centralización y control

**Racional 2:**
- **Qué:** SyncEvent enum con 5 variantes
- **Por qué:** Cubre casos de uso principales (add/remove/modify)
- **Impacto:** Fácil extensión con nuevos eventos

**Racional 3:**
- **Qué:** EventBus pub/sub
- **Por qué:** Desacopla publishers de subscribers
- **Impacto:** Mejor mantenibilidad y escalabilidad

---

## 🔗 10. RELACIONES

### 10.1 Herramientas relacionadas

**Scene Editor:**
- **Tipo de relación:** Usado por
- **Descripción:** Scene Editor usa LiveSync para sincronizar con runtime

**Runtime:**
- **Tipo de relación:** Depende de
- **Descripción:** Runtime depende de LiveSync para recibir cambios

**Asset Manager:**
- **Tipo de relación:** Usado por
- **Descripción:** Asset Manager usa LiveSync para sincronizar assets

**Event Forge:**
- **Tipo de relación:** Usado por
- **Descripción:** Event Forge usa LiveSync para sincronizar grafos

**Bitacora Manager:**
- **Tipo de relaciÃ³n:** Depende de
- **DescripciÃ³n:** LiveSync necesita Bitacora para LiveSync integration (BIT-001)
- **Feature:** LiveSync integration (Feature X)

**Play Mode:**
- **Tipo de relaciÃ³n:** Depende de
- **DescripciÃ³n:** Play Mode usa LiveSync para hot-reload

**Collaboration:**
- **Tipo de relaciÃ³n:** Usado por
- **DescripciÃ³n:** Collaboration usa LiveSync para multiplayer editing

**Hot Reload:**
- **Tipo de relaciÃ³n:** Usado por
- **DescripciÃ³n:** Hot Reload usa LiveSync para hot-reload de scripts y assets

---

## â HERRAMIENTAS INTEGRADAS (AÃ±adidas desde catÃ¡logo)

### Collaboration
- **Multiplayer editing** - EdiciÃ³n en tiempo real con mÃºltiples usuarios
- **Presence tracking** - Ver quiÃ©n estÃ¡ editando y quÃ©
- **Conflict resolution** - ResoluciÃ³n automÃ¡tica de conflictos
- **File:** `forge-editor/src/collaboration.rs`

### Hot Reload
- **Hot-reload de scripts** - Recargar scripts sin perder estado
- **Hot-reload de assets** - Recargar assets sin reiniciar
- **Estado preservado** - Mantener estado durante reload
- **File:** `forge-editor/src/hot_reload.rs`

---

**Generado automÃ¡ticamente - NO MODIFICAR FORMATO**  
**Sistema de DocumentaciÃ³n v1.0.0**  
**AI Responsable:** [AI: opencode]