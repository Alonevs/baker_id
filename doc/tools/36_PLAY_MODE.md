# ⚡ Play Mode & Live Reload 36

**Estado:** ⏳ Documentado | **Prioridad:** 🔴 Alta  
**Versión:** 1.0.0 (planned) | **Última actualización:** 2026-07-25  
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
| PlaySession | Sesión Play | play_session.rs | ⏳ Planificado |
| SnapshotManager | Snapshot | snapshot_manager.rs | ⏳ Planificado |
| InputCapture | Capturar input | input_capture.rs | ⏳ Planificado |

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

### 3.1 Arquitectura Planificada

**PlaySession** - Gestiona ciclo de vida de sesión Play:
```rust
pub struct PlaySession {
    pub snapshot: SceneSnapshot,
    pub physics_enabled: bool,
    pub input_capture: InputCapture,
    pub is_running: bool,
}

impl PlaySession {
    pub fn new(scene: &Scene) -> Self { ... }
    pub fn start(&mut self, scene: &Scene) -> Result<()> { ... }
    pub fn stop(&mut self) -> Result<()> { ... }
    pub fn update(&mut self, delta: f32) { ... }
}
```

**SnapshotManager** - Serializa/deserializa estado de escena:
```rust
pub struct SnapshotManager {
    pub scene_snapshot: SceneSnapshot,
}

impl SnapshotManager {
    pub fn take_snapshot(&self, scene: &Scene) -> SceneSnapshot { ... }
    pub fn restore_snapshot(&mut self, snapshot: SceneSnapshot) { ... }
    pub fn save_positions(&self, entities: &[Entity]) -> HashMap<EntityId, Vec2> { ... }
    pub fn load_positions(&mut self, positions: HashMap<EntityId, Vec2>) { ... }
}
```

**InputCapture** - Captura input del usuario en tiempo real:
```rust
pub struct InputCapture {
    pub keyboard_input: HashMap<KeyCode, bool>,
    pub mouse_input: MouseState,
}

impl InputCapture {
    pub fn new() -> Self { ... }
    pub fn update(&mut self, input: &UserInput) { ... }
    pub fn get_movement(&self) -> Vec2 { ... }
    pub fn is_key_pressed(&self, key: KeyCode) -> bool { ... }
}
```

### 3.2 Flujo de Datos

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Scene     │───▶│ PlaySession │───▶│ Snapshot    │───▶│  Restore   │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
      │                   │                    │                   │
      ▼                   ▼                    ▼                   ▼
  [Snapshot]          [Physics + Input]    [Serialize]        [Deseralize]
```

### 3.3 Estado Actual
- ⏳ **Documentado:** Especificaciones completas en `36_PLAY_MODE.md`
- ❌ **No implementado:** Código aún no escrito
- ⏳ **Prioridad:** Alta (tarea pendiente)

---

## 🧪 4. TESTS

### 4.1 Tests Planificados

**Unitarios:**
```rust
#[test]
fn test_take_snapshot() { ... }
#[test]
fn test_restore_snapshot() { ... }
#[test]
fn test_input_capture() { ... }
```

**Integración:**
```rust
#[test]
fn test_play_session_lifecycle() { ... }
#[test]
fn test_physics_simulation() { ... }
```

**Estado:** ⏳ Pendiente de implementación

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
| Documentación | 161 líneas | < 300 | ✅ |
| Tests | 0/0 | 10+ | ⏳ |
| Líneas de código | 0 | < 1000 | ⏳ |
| Integración | No implementado | 100% | ⏳ |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Planificado ⏳)
- [x] Documentación completa en `36_PLAY_MODE.md`
- [ ] Implementar PlaySession
- [ ] Implementar SnapshotManager
- [ ] Implementar InputCapture
- [ ] Botón Play (▶) en Toolbar
- [ ] Simulación físicas activas

### 8.2 Fase 2: Mejoras (Planificado ⏳)
- [ ] Input del usuario (teclado/ratón)
- [ ] Botón Stop (⏹)
- [ ] Restauración automática de estado
- [ ] UI de controles Play/Stop/Pause

### 8.3 Fase 3: Avanzado (Pendiente)
- [ ] Hot Reload de assets
- [ ] Debugger en tiempo real
- [ ] Exportación de grabaciones

---

**Estado:** Documentado (⏳ Pendiente implementación)  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]