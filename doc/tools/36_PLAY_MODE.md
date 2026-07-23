# ⚡ Play Mode & Live Reload 36

**Estado:** ⏳ Pendiente | **Prioridad:** 🔴 Alta  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer
Probar y simular el juego de manera interactiva en la propia ventana del Viewport del editor con snapshot de posiciones y restauración automática. Captura input del usuario (teclado/ratón).

### 1.2 Problemas que resuelve
- Testeo rápido sin recompilar
- Interacción en tiempo real
- Restauración automática de estado
- Feedback inmediato de cambios

### 1.3 Usuarios objetivo
- Diseñadores (usan para probar niveles)
- Programadores (usan para debugging)
- QA testers (usan para validar)

### 1.4 Requisitos de entrada
- Escena actual
- Snapshot de posiciones
- Input del usuario

### 1.5 Requisitos de salida
- Simulación física activa
- Input capturado
- Restauración automática al Stop

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Scene State   │───▶│  Play Mode      │───▶│  Runtime Sim    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
   [Snapshot]           [Physics + Input]      [Simulate]
```

### 2.2 Componentes principales

| Componente | Responsabilidad | Archivo | Estado |
|------------|-----------------|---------|--------|
| PlaySession | Sesión Play | play_session.rs | ❌ |
| SnapshotManager | Snapshot | snapshot_manager.rs | ❌ |
| InputCapture | Capturar input | input_capture.rs | ❌ |

### 2.3 Flujo de datos
1. Input: Click Play + Snapshot
2. Process: Activar físicas + capturar input
3. Output: Simulación en Viewport
4. Stop: Restaurar desde snapshot

### 2.4 Dependencias

**Depende de:**
- `forge-physics::PhysicsWorld` - Físicas
- `forge-scene::Scene` - Escena

**Usado por:**
- `main.rs` - Botón Play en Toolbar
- `viewport::Viewport` - Render en Play

### 2.5 Interfaz pública (API)

```rust
pub struct PlaySession {
    pub snapshot: SceneSnapshot,
    pub physics_enabled: bool,
}

impl PlaySession {
    pub fn start(&mut self, scene: &Scene) -> Result<()> { ... }
    pub fn stop(&mut self) -> Result<()> { ... }
    pub fn update(&mut self, delta: f32) { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
// TODO: Implementar
// pub struct PlaySession { ... }
```

### 3.2 Archivos creados
| Archivo | Líneas | Función | Estado |
|---------|--------|---------|--------|
| play_session.rs | 0 | Sesión Play | ❌ Pendiente |

### 3.3 Funcionalidades implementadas
- [ ] Snapshot de posiciones
- [ ] Botón Play (▶)
- [ ] Simulación físicas
- [ ] Captura input (teclado/ratón)

### 3.4 Funcionalidades pendientes (TO-DO)
- [ ] Botón Stop (⏹)
- [ ] Restaurar posiciones
- [ ] Input del usuario
- [ ] Mover jugador con físicas

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
// TODO: Implementar
#[test]
fn test_take_snapshot() { ... }
```

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
// TODO: Ejemplo
let mut session = PlaySession::new();
session.start(scene);
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | 0 | < 1000 | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Pendiente ⏳)
- [ ] Snapshot
- [ ] Botón Play
- [ ] Simulación físicas

### 8.2 Fase 2: Mejoras (Pendiente ⏳)
- [ ] Input del usuario
- [ ] Botón Stop
- [ ] Restauración

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]