# 🎭 CineGraph & Dialogue Editor 34

**Estado:** ⏳ Pendiente | **Prioridad:** 🟡 Media  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Enlazar eventos de diálogos, cinemáticas in-game y lógica condicional mediante nodos y conexiones visuales (cables Bézier). Soporta TriggerZones (burbujas verdes) en Viewport.

### 1.2 Problemas que resuelve
- Visual scripting sin código
- Diseño de flujos de diálogo
- Conexiones visuales de eventos
- TriggerZones en Viewport

### 1.3 Usuarios objetivo
- Diseñadores de juegos (usan directamente)
- Escritores (usan para diálogos)
- Programadores (usan para prototipado)

### 1.4 Requisitos de entrada
- Nodos de evento (diálogo, trigger, condicional)
- Sockets de entrada/salida
- Lienzo infinito

### 1.5 Requisitos de salida
- Grafo de eventos JSON
- TriggerZones en Viewport
- Ejecución en runtime

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Event Nodes   │───▶│  CineGraph      │───▶│  Runtime Exec   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
   [Nodes + Cables]        [Graph Eval]        [Execute Events]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| CineGraphManager | Gestor principal | cinegraph_manager.rs | ❌ |
| DialogueNode | Nodo de diálogo | dialogue_node.rs | ❌ |
| TriggerZoneNode | Nodo TriggerZone | trigger_zone_node.rs | ❌ |
| BezierCable | Cables Bézier | bezier_cable.rs | ❌ |

### 2.3 Flujo de datos
1. Input: Nodos y conexiones
2. Process: Evaluar grafo
3. Output: Eventos ejecutados

### 2.4 Dependencias

**Depende de:**
- `event_node_manager::EventNodeManager` - Manager de eventos
- `dialogue_system::DialogueSystem` - Sistema de diálogos

**Usado por:**
- `main.rs` - Integración en UI
- `runtime::GameLoop` - Ejecución

### 2.5 Interfaz pública (API)

```rust
pub struct CineGraphManager {
    pub graph: EventGraph,
}

impl CineGraphManager {
    pub fn create_dialogue_node(&mut self) -> Node { ... }
    pub fn create_trigger_zone(&mut self) -> Node { ... }
    pub fn draw_bezier_cables(&self) { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
// TODO: Implementar
// pub struct CineGraphManager { ... }
```

### 3.2 Archivos creados
| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| cinegraph_manager.rs | 0 | Gestor | ❌ Pendiente |

### 3.3 Funcionalidades implementadas
- [ ] Nodos de diálogo
- [ ] Nodos condicionales
- [ ] Cables Bézier
- [ ] TriggerZones (burbujas verdes)

### 3.4 Funcionalidades pendientes (TO-DO)
- [ ] Reutilizar EventNodeManager
- [ ] Guardar JSON en `/eventos/`
- [ ] Ejecución en runtime
- [ ] Burbujas en Viewport

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
// TODO: Implementar
#[test]
fn test_dialogue_node() { ... }
```

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
// TODO: Ejemplo
let mut manager = CineGraphManager::new();
manager.create_dialogue_node();
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Pendiente ⏳)
- [ ] Nodos visuales
- [ ] Cables Bézier
- [ ] TriggerZones

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] Reutilizar EventNodeManager
- [ ] Guardado JSON
- [ ] Ejecución runtime

---

## 🔗 10. RELACIONES

### 10.1 Herramientas relacionadas

Bitacora Manager:
- **Tipo de relación:** Usado por
- **Descripción:** EventDialogManager usa BitacoraManager para mostrar notas relacionadas con eventos y diálogos
- **API usada:** `BitacoraManager::singleton()`, `get_related_entries(&event_id)`

EventNode Manager:
- **Tipo de relación:** Usado por
- **Descripción:** EventNode Manager depende de CineGraph Manager para grafos

CineGraph Manager:
- **Tipo de relación:** Usado por
- **Descripción:** CineGraph Manager depende de EventNodeManager para nodos

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]