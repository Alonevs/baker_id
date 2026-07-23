# 🏗️ ARQUITECTURA FORGE SDK 2D

---

## 📐 VISIÓN GENERAL

Motor de juego 2D modular en Rust con arquitectura basada en componentes y workspace.

```
┌─────────────────────────────────────────────────────────┐
│                      forge-workspace                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │ forge-types │  │ forge-editor│  │ forge-runtime│    │
│  │ (compartido)│  │ (IDE visual) │  │ (juego)     │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │ forge-scene │  │ forge-event │  │ forge-dialog│    │
│  │ (niveles)   │  │ (eventos)   │  │ (diálogos)  │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │ forge-panel │  │ forge-undo  │  │ forge-map   │     │
│  │ messaging   │  │ redo        │  │ cart        │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
└─────────────────────────────────────────────────────────┘
```

---

## 🗂️ ESTRUCTURA DE CRATES

### `forge-types` - Tipos compartidos
**Propósito:** Definir tipos de datos compartidos entre todos los crates.

**Contenido:**
- `Vec2`, `Mat4`, `Color`, `Vector` - Matemáticas básicas
- `NodeId`, `EntityId` - Identificadores únicos
- `Transform`, `Component` - Datos de componentes
- `Serializable`, `Deserialize` - Traits para serialización

**Estado:** ✅ Implementado

---

### `forge-scene` - Datos de escena
**Propósito:** Estructuras de datos para niveles y escenas.

**Contenido:**
- `Scene` - Escena completa con entidades y componentes
- `Entity` - Entidad con ID, transform, componentes
- `Component` - Componentes reutilizables
- `Layer` - Capas en una escena

**Estado:** ✅ Implementado

---

### `forge-event` - Sistema de eventos
**Propósito:** Sistema de eventos para comunicación entre componentes.

**Contenido:**
- `Event` - Tipo base de eventos
- `EventBus` - Pub/Sub para eventos
- `EventFilter` - Filtrado de eventos
- `EventQueue` - Cola de eventos asíncrona

**Estado:** ✅ Implementado

---

### `forge-dialogue` - Diálogos
**Propósito:** Sistema para diálogos y narración.

**Contenido:**
- `Dialogue` - Diálogo completo
- `Node` - Nodo de diálogo
- `Choice` - Opciones en diálogo
- `Transition` - Transiciones entre nodos

**Estado:** ✅ Implementado

---

### `forge-editor` - IDE visual
**Propósito:** Editor visual para crear juegos sin código.

**Contenido:**
- `Viewport` - Vista 2D de la escena
- `Inspector` - Panel de inspección de entidades
- `Hierarchy` - Árbol de entidades
- `Toolbar` - Barra de herramientas

**Estado:** ✅ Implementado

---

### `forge-runtime` - Runtime del juego
**Propósito:** Ejecución del juego.

**Contenido:**
- `GameLoop` - Bucle principal del juego
- `Update` - Actualización de entidades
- `Render` - Renderizado
- `State` - Estados del juego

**Estado:** 🔄 En progreso

---

### `forge-panel-messaging` - Eventos entre paneles
**Propósito:** Sistema centralizado de eventos entre paneles del editor.

**Contenido:**
- `EventBus` - Pub/Sub para eventos del editor
- `PanelOpen` - Evento cuando se abre un panel
- `PanelClose` - Evento cuando se cierra un panel
- `ViewportResize` - Evento de redimensionamiento
- `AssetLoaded` - Evento de carga de assets
- `SaveRequested` - Evento de guardado
- `SceneLoaded` - Evento de carga de escena
- `EntitySelected` - Evento de selección de entidad
- `PropertyChanged` - Evento de cambio de propiedad

**Estado:** ✅ Implementado

---

### `forge-undo-redo` - Undo/Redo
**Propósito:** Sistema completo de deshacer/rehacer.

**Contenido:**
- `UndoStack` - Pila con límite configurable (100 por defecto)
- `UndoManager` - Gestor con serialización JSON
- `UndoableAction` - Acción que puede ser deshaceda
- `History` - Historial de cambios

**Estado:** ✅ Implementado

---

### `forge-map-cart` - Formato .map
**Propósito:** Formato para archivos de nivel.

**Contenido:**
- `MapFile` - Archivo de nivel
- `Layer` - Capa del nivel
- `Tile` -.Tile en el nivel
- `Entity` - Entidad en el nivel
- `Component` - Componente en el nivel

**Estado:** ✅ Implementado

---

### `forge-compiler` - Compiler con QA/fuzzer
**Propósito:** Compilador de scripts con verificación.

**Contenido:**
- `Compiler` - Compilador principal
- `Parser` - Parser de scripts
- `TypeChecker` - Verificador de tipos
- `Optimizer` - Optimizador de código
- `Fuzzer` - Fuzzer para testing

**Estado:** 🔄 En progreso

---

## 🔄 COMUNICACIÓN ENTRE CRATES

### Eventos (0-A)
```
Panel Editor ──[PanelOpen]──> EventBus ──[PanelOpen]──> Script Editor
Panel Inspector ──[ViewportResize]──> EventBus ──[ViewportResize]──> Scene Viewport
Panel Asset ──[AssetLoaded]──> EventBus ──[AssetLoaded]──> Plugin System
```

### Datos (0-B)
```
Scene ──[get_entity]──> Editor ──[inspect]──> Inspector ──[update]──> Scene
Entity ──[get_component]──> Component Editor ──[modify]──> Scene ──[save]──> .map
```

### Estado (0-C)
```
UndoManager ──[push_action]──> Scene ──[get_state]──> Serialization ──[save]──> JSON
```

---

## 📊 PATRONES DE DISEÑO

### 1. Component-Based Architecture
```
Entity
├── Transform (x, y, rotation, scale)
├── Sprite (image, flip_h, flip_v)
├── Collider (type: box/circle, size)
└── Script (script_id, parameters)
```

### 2. Event-Driven Communication
```
Publisher ──[Event]──> EventBus ──[Filter]──> Subscribers
```

### 3. Undo/Redo Pattern
```
Action ──[undo]──> UndoStack ──[redo]──> Action
```

### 4. Workspace Pattern
```
Workspace ──[open]──> Project ──[load]──> Scene ──[render]──> Viewport
```

---

## 💻 IDE/SDK - CONFIGURACIÓN INTEGRADA

### PUNTO 1: Flujo de Bienvenida y Asistente de Género

**Pantalla Modal de Inicio:**
1. **Selector de Ruta:** [📁 Nuevo Proyecto] o [📂 Cargar Proyecto]
2. **Interruptor de Género:** 4 perfiles iniciales:
   - Isométrico
   - Ortogonal
   - Sprites Libres
   - Lienzo Rígido
3. **Inicialización:** Configuración automática del motor + activación "Modo Tutor"

### PUNTO 2: "Modo Tutor" Interactivo

**Ubicación:** Pestaña flotante translúcida en esquina superior derecha

**Comportamiento UX:**
- Checklist dinámico que brilla para guiar primeros pasos
- **Paso 1 (Datos):** Arrastrar primer PNG de fondo/spritesheet
- **Paso 2 (Posicionamiento):** Seleccionar asset y arrastrar al Canvas Central
- **Paso 3 (Mecánicas):** Pincel de Físicas para trazar líneas de colisión
- **Botón de Silencio [🔕 Ocultar Guía]:** Animación fluida hacia el borde

### PUNTO 3: Viewport Camaleónico

**Ubicación:** Centro de pantalla (consume todo espacio dinámico restante)

**Comportamiento según interruptor del Asistente:**
- **Modo A (Rejilla Cuadrada):** Grid tradicional
- **Modo B (Isométrica):** Grid isométrico 2:1
- **Modo C (Lienzo Libre):** Sin grid, posicionamiento manual

---

## 🗺️ WORDS TÉCNICOS - ROADMAP DE ARQUITECTURA

### WORD 1: Gestor de Tilesets, Paletas e Importador ✅ YA HECHO
- **Propósito:** La "aduana" de gráficos. Guillotina de píxeles paramétrica (cortar en celdas cuadradas u rombos isométricos), pintor de colisiones con Flags (`SÓLIDO = true`), enlazador de bucles animados e importador/filtro restrictivo de paletas indexadas de 256 colores.
- **Estado:** Implementado en `src/ui/asset_browser.rs` (650 líneas) y optimizado.
- **Salida:** Archivo `.toml` de configuración en PC y binario comprimido `.sprites` para el cartucho.

### WORD 2: Núcleo Gráfico, Escalado y Mapeo de Inputs ⏳ PENDIENTE
- **Propósito:** Lienzo vivo de ejecución. Controla el framebuffer rígido de 960x540 en pantalla, la lógica de escalado entero sin aliasing en monitores modernos y el mapeo nativo de inputs físicos de gamepad/mando a través de hilos paralelos de latencia cero.
- **Dependencias:** forge-types.
- **Output:** Viewport base con renderizado a 60fps y lectura de inputs directos.

### WORD 3: Editor de Mapas Isométrico y Sistema Drag & Drop ⏳ PENDIENTE
- **Propósito:** Integración de los Words 1 y 2 en el Viewport. Rejilla matemática isométrica 2:1 (inclinación de 26.565°). Lógica de raycasting e imantación magnética instantánea de baldosas al lienzo y cálculo dinámico de altura virtual (Eje Z).
- **Dependencias:** WORD 1, WORD 2.
- **Output:** Editor con pinceles de pintado por arrastre y altura de nodos.

### WORD 4: Jerarquía de la Escena y Árbol de Nodos ⏳ PENDIENTE
- **Propósito:** Estructura de datos lógica en memoria RAM. Modelo jerárquico de nodos, cálculo de transformaciones relativas recursivas de padres a hijos y algoritmo del pintor (Z-Sorting) para ordenar y tapar sprites en falso 3D/2.5D.
- **Dependencias:** WORD 1, WORD 2, WORD 3.
- **Output:** SceneTree real sincronizado con `Vec<Arc<NodeData>>`.

### WORD 5: Asistente del Baker 3D y Gestor de Sockets ⏳ PENDIENTE
- **Propósito:** Conversión automatizada de mallas 3D `.gltf` / `.glb` a hojas de sprites 2D en 8 direcciones angulares. Asistente con trimming de loops de animación, compresión VRAM y cálculo de Sockets (puntos de anclaje de armas/objetos 3D a píxeles 2D frame a frame).
- **Dependencias:** WORD 1, WORD 4.
- **Output:** Sprite baker con renderizado off-screen GPU integrado.

### WORD 6: Línea de Tiempo y el Cine Graph ⏳ PENDIENTE
- **Propósito:** Orquestación y guionización visual de secuencias. Línea de tiempo (Timeline) con keyframes de animación, pistas de disparadores lógicos (hitboxes) y audio espacial posicional, y editor visual del flujo narrativo y cinemático mediante cables Bézier (Event Forge).
- **Dependencias:** WORD 4, WORD 5.
- **Output:** Lógica secuencial interactiva empaquetable en el JSON del cartucho.

**Orden de dependencias de desarrollo:** WORD 1 ➔ WORD 2 ➔ WORD 3 ➔ WORD 4 ➔ WORD 5 ➔ WORD 6

---

## 🎨 UI/UX PRINCIPALES

### Editor de Mapas Isométrico
- **Rejilla:** Matemática isométrica con ratio 2:1 (inclinación de 45°)
- **Drag & Drop:** Arrastrar tiles creados en WORD 1
- **Imantación:** Magnética para alineación precisa
- **Altura Virtual:** Cálculo del Eje Z para profundidad

### Sistema de Nodos
- **Visual:** Caja y cables curvados en tiempo real
- **Framework:** egui_nodes
- **Función:** Lógica del flujo de historia, cinemáticas, ramificaciones de misiones

### Base de Datos Narrativa
- **Formato:** CSV estructural
- **Motor:** serde + csv
- **Gestión:** Automatizada

---

## 📚 REFERENCIAS EXTERNAS

| Engine | URL |
|--------|-----|
| Unity | https://docs.unity.com |
| Unreal | https://docs.unrealengine.com |
| Godot | https://docs.godotengine.org |
| GameMaker | https://manual.gamemaker.io |
| Defold | https://defold.com/manuals |
| Construct | https://www.construct.net/en/make-games/manual |
| Cocos | https://docs.cocos.com |
| RPG Maker MV | https://kinoar.github.io/rmmv-doc-web |
| Defold Docs | https://github.com/defold/doc |
| O3DE | https://docs.o3de.org |
| Bevy | https://bevyengine.org/learn/book/introduction/ |
| Stride 3D | http://doc.stride3d.net |
| ProwlEngine | https://github.com/ProwlEngine/Prowl |

---

## 🔧 CONFIGURACIÓN

### `Cargo.toml`
```toml
[workspace]
members = [
    "crates/forge-types",
    "crates/forge-scene",
    "crates/forge-event",
    "crates/forge-dialogue",
    "crates/forge-editor",
    "crates/forge-runtime",
    "crates/forge-panel-messaging",
    "crates/forge-undo-redo",
    "crates/forge-map-cart",
    "crates/forge-compiler",
]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### `build.rs`
```rust
fn main() {
    // Generate version info
    // Generate shader bindings
    // Generate type definitions
}
```

---

## 📈 ESCALABILIDAD

### Memoria
- **Heap:** Pools para entidades y componentes
- **Stack:** Datos temporales pequeños
- **External:** Assets en memoria externa

### CPU
- **Single-thread:** Game loop principal
- **Multi-thread:** Physics, rendering, audio
- **Async:** I/O, networking, serialization

### Storage
- **RAM:** Active scene y runtime
- **Disk:** .map files, assets, configs
- **Cloud:** Collaboration (future)

---

## 🧬 GESTIÓN DE DATOS Y PREFABS (EXCEL RELACIONAL)

### Modelo Relacional de Assets (Evitar Duplicados)
Para optimizar el uso de VRAM y memoria en el cartucho de la portátil RISC-V, los recursos gráficos (texturas, Spritesheets) y de audio (`.wav`) se almacenan de forma unificada en un directorio global compartible. Las entidades del juego en RAM no duplican estos assets, sino que hacen referencia a ellos a través de un **ID de Entidad Relacional** y una clave compartida. Esto permite, por ejemplo, tener un "Ropero Global" donde se almacena una sola vez un sprite de armadura, que el motor pinta dinámicamente encima de múltiples personajes al vuelo.

### Sistema de Arquetipos y Herencia de Prefabs
- **Prefabs Base (Moldes):** Permite configurar una plantilla de actor con componentes gráficos, scripts de comportamiento, colisionadores y radios de audio en un archivo `.prefab` JSON reutilizable.
- **Instancias Locales (Herencia Pasiva):** Al colocar múltiples copias en el mapa, estas heredan todas las propiedades del molde base de forma automática.
- **Modificación Contextual:** Habilita al diseñador a cambiar las propiedades individuales de un actor específico en el escenario (ej. cambiar el diálogo de un guardia concreto o el tamaño de su trigger) sin romper el enlace con el molde general, actualizando el resto de instancias si el prefab base se edita.

### Notas de Diseño Contextuales y Registro de Cambios
Cada entidad del escenario contiene un bloque descriptor de metadatos en fase de diseño que permite al usuario añadir comentarios breves sobre las modificaciones realizadas. Este log registra de forma automática el cambio y la fecha, ayudando al control de versiones de diseño dentro de la propia IDE sin que esta información de depuración se inyecte en el binario final del cartucho.

---

**Última actualización:** 2026-07-23  
**AI:** [AI: opencode]
