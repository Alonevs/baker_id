# 📐 MANUAL DE DISEÑO VISUAL Y UX (UX_MANUAL.md)

Este documento es la especificación visual y de interacción obligatoria para la interfaz del **Forge Editor**. Define los estándares estéticos, controles de ventanas, comportamientos de lienzos (2D/3D) y el sistema interactivo de burbujas (Gizmos) para mantener consistencia de desarrollo en el ecosistema.

---

## 🎨 1. FILOSOFÍA VISUAL Y "LOOK & FEEL"

### 1.1 Paleta de Colores del Tema Oscuro (Dark Mode)
Para mitigar la fatiga ocular, toda la interfaz adopta una paleta Flat Design neutra con acentos de alta visibilidad:
- **Fondo Principal (Lienzo/Vacíos):** `#121212` (Gris casi negro mate).
- **Fondo de Paneles y Contenedores:** `#1E1E1E` (Gris carbón).
- **Fondo de Campos de Texto e Inputs:** `#2D2D2D` (Gris pizarra medio).
- **Texto Primario (Títulos/Datos):** `#E0E0E0` (Blanco roto).
- **Texto Secundario (Descripciones):** `#888888` (Gris medio).
- **Color de Acento (Selección/Activos):** `#E67E22` (Naranja Rust corporativo).
- **Alerta / Peligro (Hitboxes/Eliminar):** `#C0392B` (Rojo carmesí).
- **Confirmación / Éxito (Terminado):** `#27AE60` (Verde esmeralda).

### 1.2 Bordes, Esquinas y Tipografía
- **Bordes:** Líneas finas de 1 píxel de grosor color `#333333` para delimitación de ventanas.
- **Esquinas (Corner Radius):** Redondeado sutil de 4 píxeles en botones, inputs y tarjetas.
- **Tipografía:** Sans-Serif de tipo geométrico (Inter, Fira Sans o Roboto). Escala:
  - Títulos de paneles: 14pt (Negrita).
  - Etiquetas e inputs: 11pt (Regular).
  - Textos de ayuda / Tooltips: 9pt (Itálica).

### 1.3 Sistema de Paneles Responsivo (Docking)
El editor se monta sobre un árbol de particiones horizontales y verticales separadas por divisores (**Splitters**) interactivos de 4 píxeles de ancho (cursor cambia a doble flecha `↔` / `↕` para arrastrar).
- **Restricciones de Ancho Mínimo:** Paneles laterales ≥ 200px, Panel inferior ≥ 150px.
- **Colapso Rápido (Foldable Panels):** Botón triangular (`▼` / `►`) en la esquina superior izquierda de cada panel para encogerlo instantáneamente hacia el borde.

---

## 📂 2. COMPORTAMIENTO DE LOS PANELES PRINCIPALES

### 2.1 El Explorador del Proyecto (Asset Browser)
Disposición visual en **dos columnas** que sincroniza los cambios en disco:
- **Columna Izquierda (Árbol de Carpetas):** Vista jerárquica desplegable de subcarpetas del proyecto. Bloqueo estricto de carpetas raíz inmutables (`/Audio`, `/Modelos3D`, `/Escenarios`, `/Fondos`).
- **Columna Derecha (Visor de Contenido):** Cuadrícula o lista compacta con buscador dinámico en caliente (Fuzzy Search) y controles de historial (Atrás `⬅️`, Adelante `➡️`, Inicio `🏠`).
- **Drag and Drop:** Permite arrastrar archivos gráficos del explorador al Viewport central para instanciarlos automáticamente, mostrando un cursor de prohibido (`🚫`) si se suelta sobre zonas no válidas.

### 2.2 La Jerarquía de la Escena (Scene Tree)
Lista compacta vertical de los nodos vivos e instanciados en memoria RAM para el nivel actual.
- **Nodos:** Cada fila (20px de alto) contiene flecha de colapso, icono del tipo de nodo, nombre, y un **icono de ojo** en el extremo derecho para alternar visibilidad en el editor.
- **Parentesco (Parenting):** Arrastrar un nodo sobre otro para subordinarlo. El nodo hijo hereda de forma estricta las transformaciones (coordenadas relativas acumuladas de posición, rotación y escala) del padre.
- **Sorting Layers (Capas de Renderizado):** Reordenamiento de filas mediante arrastre vertical para definir qué sprites se dibujan por delante de otros en el mapa (Algoritmo del pintor).

---

## 👁️ 3. EL VIEWPORT CENTRAL (MODOS DE ENTORNO)

El visor del editor conmuta dinámicamente entre dos modos en función del tipo de recurso activo:

### 3.1 Modo Importador 3D (Visor GLTF)
Se inicializa mediante la API `wgpu` de Rust al abrir un archivo `.gltf` o `.glb` en el explorador:
- **Entorno:** Fondo degradado de `#2A2A2A` a `#1A1A1A` con una rejilla infinita de suelo.
- **Visualización (Overlays):** Botones flotantes superiores para alternar entre **Modo Sólido** (texturas), **Modo Alambre** (triángulos/wireframe) y **Modo Esqueleto** (huesos de rigging en rayos X).
- **Controles de Cámara:** Click derecho mantenido + arrastre para rotar (Orbit), Shift + click derecho para trasladar (Pan), y rueda del ratón para Zoom.
- **Timeline 3D:** Barra de tiempo inferior para previsualizar animaciones por huesos con botones Play, Pausa y Loop, e interacción de arrastre (Scrubbing).

### 3.2 Modo Editor de Mapas (Lienzo 2D/2.5D)
Lienzo infinito con resolución virtual fija de **960x540**:
- **Rejilla Isométrica 2:1:** Rombos con ángulo de inclinación de 26.565° (2 píxeles de ancho por 1 de alto). El tamaño base de la celda es de `64x32` píxeles.
- **Snapping Magnético:** Los sprites colocados se imantan automáticamente a la coordenada de rejilla bajo el ratón.
- **Eje Z de Altura:** El desarrollador puede elevar entidades. El editor dibuja una línea guía de puntos verdes para rastrear la celda del suelo bajo el sprite flotante.
- **Gizmos Universales (Teclas Rápidas):**
  - `W` (Movimiento): Muestra flechas de ejes (Roja: X, Azul: Y, Verde: Z de altura).
  - `E` (Rotación): Dibuja un círculo azul en la base para rotar al personaje en las 8 direcciones nativas.
  - `R` (Escalado): Cubos de deformación y escalado uniforme con interpolación Nearest Neighbor.

---

## 📐 4. EL INSPECTOR DE COMPONENTES Y ASISTENTES

### 4.1 Inspector Dinámico
Muestra las propiedades de la entidad seleccionada en una pila de tarjetas colapsables.
- **Sliders Virtuales:** Habilita el arrastre lateral con el ratón directamente sobre las etiquetas de texto de propiedades numéricas (ej: `X:` o `Y:`) para incrementar o decrementar valores rápidamente mientras se observa el cambio en caliente en el Viewport.

### 4.2 Asistente del Baker 3D (Horneado de Sprites)
Pestañas de configuración secuenciales en la columna del Inspector para renderizar sprites 2D a partir del modelo 3D activo:
- **Pestaña 1 (Animaciones):** Mapeo de clips de Blender y corte de intervalos en segundos.
- **Pestaña 2 (Captura):** Rosa de los vientos interactiva de 8 direcciones para activar/desactivar cámaras de captura. Botón de espejo (Mirror) automático para reducir a la mitad la VRAM final.
- **Pestaña 3 (Renderizado):** Resolución de la hoja de sprites (64, 128, 256) y FPS de muestreo (5, 9, 12, 24).
- **Proceso (Bake):** Botón que inicia la renderización por GPU bloqueando la edición (Modal Lock) y mostrando una barra de progreso informativa.

---

## 🔮 5. EL SISTEMA DE BURBUJAS INTERACTIVAS (GIZMOS 2D)

Para configurar lógica espacial sin tocar código, el editor dibuja contornos circulares/elípticos translúcidos y manipulables directamente en el Viewport mediante Clic Derecho ➔ Componentes:

```
           [BURBUJA DE INTERACCIÓN]
                     /\
                   /    \  <-- Nodo de Radio de Atenuación / Activación
                 /   ●    \
                 \ (Objeto)\
                   \    /
                     \/
```

### 🔵 5.1 Burbuja Azul: Vector de Audio Posicional
- **Función:** Define el radio de escucha de un altavoz virtual.
- **Interacción:** El desarrollador estira la burbuja visualmente sobre el mapa. Durante el juego, la distancia en píxeles del personaje al centro de la burbuja escala dinámicamente el volumen del audio y desplaza el paneo estéreo (izquierda/derecha).

### 🟢 5.2 Burbuja Verde: Triggers y Disparadores Lógicos
- **Función:** Zona de colisión invisible que reacciona de forma automática al contacto físico del jugador.
- **Interacción:** Se deforma con el ratón para cubrir pasillos o entradas. Al cruzar el borde, se gatilla un evento asociado (ej. inicio de cinemática, lluvia de partículas o cambio de mapa).

### 	🟡 5.3 Burbuja Amarilla: Zonas de Diálogo e Interacción Voluntaria
- **Función:** Zona que requiere una acción voluntaria del jugador (cercanía + presionar botón A).
- **Interacción:** Al entrar el personaje en la burbuja, el motor muestra flotando en pantalla un icono dinámico (Prompt visual del Botón A). Al presionarlo, congela el control del juego y lanza el diálogo o cofre asociado.

---

**Última actualización:** 2026-07-23  
**AI:** [AI: opencode]  
