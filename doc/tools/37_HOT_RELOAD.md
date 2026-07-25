# Fase 37: Hot Reload Panel

## Objetivo
Implementar un panel de Hot Reload con lógica real para hot reload de scripts en tiempo real.

## Estructura

### Módulos
- `hot_reload.rs` - HotReloadManager para gestionar cambios de scripts
- `hot_reload_panel.rs` - UI del panel de hot reload

### Componentes

#### HotReloadManager
```rust
pub struct HotReloadManager {
    pub script_executor: Option<ScriptExecutor>,
    pub hot_reload_panel: HotReloadPanel,
    pub pending_changes: Vec<PendingChange>,
    pub is_hot_reload_enabled: bool,
}
```

#### HotReloadPanel
```rust
pub struct HotReloadPanel {
    pub selected_file: Option<String>,
    pub preview_result: Option<String>,
    pub preview_error: Option<String>,
    pub diff_view: HotReloadDiffView,
    pub show_version_history: bool,
    pub debounce_count: u64,
    pub reload_status: ReloadStatus,
}
```

## Métodos

### HotReloadManager
- `new()` - Crea un nuevo HotReloadManager
- `enable()` / `disable()` - Habilita/deshabilita hot reload
- `register_change()` - Registra un cambio en un script
- `process_pending_changes()` - Procesa los cambios pendientes
- `get_panel()` / `get_panel_mut()` - Accede al panel

### HotReloadPanel
- `new()` - Crea un nuevo panel
- `ui(ctx)` - Renderiza la UI del panel
- `update(dt)` - Actualiza el panel con delta time
- `render_status()` - Renderiza el estado de carga
- `render_file_selector()` - Renderiza selector de archivos
- `render_diff_view()` - Renderiza vista de diff
- `render_preview()` - Renderiza preview del script
- `render_actions()` - Renderiza botones de acción

## Tests

### Tests de HotReloadPanel
1. `test_hot_reload_panel_new` - Crea panel nuevo
2. `test_hot_reload_panel_default` - Crea panel por default
3. `test_hot_reload_panel_update` - Actualiza panel
4. `test_hot_reload_panel_reload_status` - Cambia estado de reloading a reloaded
5. `test_hot_reload_panel_diff_view` - Verifica diff view

### Tests de HotReloadManager
6. `test_hot_reload_manager_new` - Crea manager nuevo
7. `test_hot_reload_manager_enable_disable` - Habilita/deshabilita
8. `test_hot_reload_manager_register_change` - Registra cambio
9. `test_hot_reload_manager_process_changes` - Procesa cambios
10. `test_hot_reload_panel_get_stats` - Obtiene stats

## Integración

### En ForgeEditorApp
```rust
pub struct ForgeEditorApp {
    pub hot_reload_panel: HotReloadPanel,
}
```

### UI del Panel
- **Status**: Muestra estado (Ready/Reloading/Reloaded/Error)
- **File Selector**: Lista de scripts disponibles para hot reload
- **Diff View**: Muestra cambios entre versiones
- **Preview**: Muestra resultado de ejecución del script
- **Actions**: Botones Reload/Stop/Clear

## Estado

| Componente | Estado | Tests |
|------------|--------|-------|
| HotReloadManager | ✅ Implementado | - |
| HotReloadPanel | ✅ Implementado | 5/5 |
| Tests | ✅ 11/11 | 100% |
| Build | ✅ 0 errors | - |
| **Total** | ✅ **100%** | **11/11** |

## Next Step
Integrar Hot Reload con File System watcher para detectar cambios en tiempo real.
