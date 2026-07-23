# 🚨 Inspector Físico + Gizmos 33

**Estado:** ⏳ Pendiente | **Prioridad:** 🟡 Media  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Añadir colisiones a entidades y visualizarlas en tiempo real con Gizmos (contornos translúcidos: Rojo para AABB, Azul para círculos, Amarillo para polígonos).

### 1.2 Problemas que resuelve
- Visualiza colisiones antes de ejecutar
- Facilita ajuste de colisiones
- Elimina debugging en runtime
- Mejora UX para configuración física

### 1.3 Usuarios objetivo
- Diseñadores de niveles (usan directamente)
- Programadores (usan para debugging)
- QA testers (usan para validar colisiones)

### 1.4 Requisitos de entrada
- Entidad con componente Collider/PhysicsBody
- Tipo de colisión (AABB, Circle, Polygon)
- Propiedades físicas

### 1.5 Requisitos de salida
- Colisión visualizada en Viewport
- Componente Collider/PhysicsBody actualizado
- Preview en tiempo real

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Inspector     │───▶│  Physics Body   │───▶│  Gizmos View    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
   [Config]              [Collision Type]      [Visual Render]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| PhysicsInspector | Inspector físico | physics_inspector.rs | ❌ |
| GizmoRenderer | Render Gizmos | gizmo_renderer.rs | ❌ |
| ColliderConfig | Configurar colisión | collider_config.rs | ❌ |

### 2.3 Flujo de datos
1. Input: Configuración en Inspector
2. Process: Aplicar a componente Collider
3. Output: Gizmos visualizados en Viewport

### 2.4 Dependencias

**Depende de:**
- `forge-physics::Collider` - Componente Collider
- `forge-physics::PhysicsBody` - Componente PhysicsBody
- `egui` - Render Gizmos

**Usado por:**
- `main.rs` - Integración en Inspector
- `viewport::Viewport` - Render en tiempo real

### 2.5 Interfaz pública (API)

```rust
pub struct PhysicsInspector {
    pub selected_entity: Option<EntityId>,
}

impl PhysicsInspector {
    pub fn set_collision_type(&mut self, entity: EntityId, type: CollisionType) { ... }
    pub fn set_mass(&mut self, entity: EntityId, mass: f32) { ... }
    pub fn draw_gizmos(&self, viewport: &Viewport) { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
// TODO: Implementar
// pub struct PhysicsInspector { ... }
```

### 3.2 Archivos creados
| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| physics_inspector.rs | 0 | Inspector | ❌ Pendiente |

### 3.3 Funcionalidades implementadas
- [ ] Configurar tipo de colisión
- [ ] Configurar propiedades físicas
- [ ] Visualizar AABB (Rojo)
- [ ] Visualizar Circle (Azul)

### 3.4 Funcionalidades pendientes (TO-DO)
- [ ] Visualizar Polygon (Amarillo)
- [ ] Tiempo real
- [ ] Integración forge-physics
- [ ] Gizmos en Viewport

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
// TODO: Implementar
#[test]
fn test_draw_aabb() { ... }
```

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
// TODO: Ejemplo
let mut inspector = PhysicsInspector::new();
inspector.set_collision_type(entity_id, CollisionType::AABB);
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Pendiente ⏳)
- [ ] Inspector Físico
- [ ] Gizmos AABB/Circle
- [ ] Componentes Collider

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] Polygon gizmos
- [ ] Tiempo real
- [ ] Integración physics

---

## ⚠️ HERRAMIENTAS INTEGRADAS (Añadidas desde catálogo)

### Physics 2D
- **Colliders (AABB, Circle)** - Detectores de colisión
- **Body dynamics** - Estático, Cinemático, Dinámico
- **Collision events** - Pre-solve, Post-solve, Begin, End
- **File:** `forge-editor/src/physics_2d.rs`

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]