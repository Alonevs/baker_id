use crate::ui_system::{
    UiComponentType, UiComponentState, UiPos, UiSize, UiRect, UiColor,
    UiFont, UiTextAlign, UiVerticalAlign, UiBorder, UiShadow,
};

/// Trait para renderizador de UI
pub trait Renderer {
    fn draw(&mut self, _x: f32, _y: f32, _color: UiColor, _width: f32, _height: f32);
    fn draw_text(&mut self, _x: f32, _y: f32, _text: &str, _color: UiColor);
}

/// Base trait para todos los componentes UI
pub trait UiComponent {
    fn id(&self) -> UiComponentId;
    fn update(&mut self, dt: f32);
    fn render(&self, renderer: &mut dyn Renderer);
    fn handle_input(&mut self, event: &InputEvent) -> Option<InputAction>;
    fn set_position(&mut self, pos: UiPos);
    fn set_size(&mut self, size: UiSize);
    fn set_visible(&mut self, visible: bool);
    fn is_visible(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn is_enabled(&self) -> bool;
    fn state(&self) -> UiComponentState;
    fn is_mouse_over(&self, position: &UiPos) -> bool;
}

/// Componente Button
#[derive(Debug, Clone)]
pub struct Button {
    pub id: UiComponentId,
    pub label: String,
    pub position: UiPos,
    pub size: UiSize,
    pub visible: bool,
    pub enabled: bool,
    pub state: UiComponentState,
    pub color: UiColor,
    pub hover_color: UiColor,
    pub press_color: UiColor,
    pub font: UiFont,
    pub border: UiBorder,
    pub shadow: UiShadow,
    pub on_click: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Button {
    pub fn new(id: UiComponentId, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
            position: UiPos::zero(),
            size: UiSize::new(100.0, 40.0),
            visible: true,
            enabled: true,
            state: UiComponentState::Idle,
            color: UiColor::white(),
            hover_color: UiColor::new(0.9, 0.9, 0.9, 1.0),
            press_color: UiColor::new(0.8, 0.8, 0.8, 1.0),
            font: UiFont::medium(),
            border: UiBorder::new(1.0, UiColor::black()),
            shadow: UiShadow::new(2.0, -2.0, 5.0, UiColor::black()),
            on_click: None,
        }
    }
    
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }
    
    pub fn set_color(&mut self, color: UiColor) {
        self.color = color;
    }
    
    pub fn set_font(&mut self, font: UiFont) {
        self.font = font;
    }
    
    pub fn set_on_click(&mut self, callback: Box<dyn Fn() + Send + Sync>) {
        self.on_click = Some(callback);
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new(UiComponentId::new(1), "Button")
    }
}

impl UiComponent for Button {
    fn id(&self) -> UiComponentId {
        self.id
    }
    
    fn update(&mut self, _dt: f32) {
        // Actualizar estado basado en input
    }
    
    fn render(&self, _renderer: &mut dyn Renderer) {
        // Renderizar botón
    }
    
    fn handle_input(&mut self, event: &InputEvent) -> Option<InputAction> {
        if !self.visible || !self.enabled {
            return None;
        }
        
        match event {
            InputEvent::MouseMove { position, .. } => {
                // Check if mouse is over button
                if self.is_mouse_over(position) {
                    self.state = UiComponentState::Hovered;
                    Some(InputAction::None)
                } else {
                    self.state = UiComponentState::Idle;
                    Some(InputAction::None)
                }
            }
            InputEvent::MouseButtonPressed { button, .. } => {
                if *button == MouseButton::Left && self.state == UiComponentState::Hovered {
                    self.state = UiComponentState::Pressed;
                    if let Some(callback) = &self.on_click {
                        callback();
                    }
                    Some(InputAction::Clicked)
                } else {
                    Some(InputAction::None)
                }
            }
            InputEvent::MouseReleased { .. } => {
                self.state = UiComponentState::Idle;
                Some(InputAction::None)
            }
            _ => Some(InputAction::None),
        }
    }
    
    fn set_position(&mut self, pos: UiPos) {
        self.position = pos;
    }
    
    fn set_size(&mut self, size: UiSize) {
        self.size = size;
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    fn is_visible(&self) -> bool {
        self.visible
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn state(&self) -> UiComponentState {
        self.state
    }
    
    fn is_mouse_over(&self, position: &UiPos) -> bool {
        position.x >= self.position.x &&
        position.x <= self.position.x + self.size.width &&
        position.y >= self.position.y &&
        position.y <= self.position.y + self.size.height
    }
}

/// Componente Panel
#[derive(Debug, Clone)]
pub struct Panel {
    pub id: UiComponentId,
    pub title: Option<String>,
    pub position: UiPos,
    pub size: UiSize,
    pub visible: bool,
    pub enabled: bool,
    pub state: UiComponentState,
    pub background_color: UiColor,
    pub border_color: UiColor,
    pub padding: UiSize,
    pub children: Vec<Box<dyn UiComponent>>,
}

impl Panel {
    pub fn new(id: UiComponentId, title: Option<&str>) -> Self {
        Self {
            id,
            title: title.map(|s| s.to_string()),
            position: UiPos::zero(),
            size: UiSize::new(200.0, 150.0),
            visible: true,
            enabled: true,
            state: UiComponentState::Idle,
            background_color: UiColor::new(0.2, 0.2, 0.2, 1.0),
            border_color: UiColor::new(0.4, 0.4, 0.4, 1.0),
            padding: UiSize::new(10.0, 10.0),
            children: Vec::new(),
        }
    }
    
    pub fn add_child(&mut self, child: Box<dyn UiComponent>) {
        self.children.push(child);
    }
    
    pub fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
    }
    
    pub fn get_child_count(&self) -> usize {
        self.children.len()
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self::new(UiComponentId::new(1), None)
    }
}

impl UiComponent for Panel {
    fn id(&self) -> UiComponentId {
        self.id
    }
    
    fn update(&mut self, dt: f32) {
        // Actualizar hijos
        for child in &mut self.children {
            child.update(dt);
        }
    }
    
    fn render(&self, renderer: &mut dyn Renderer) {
        // Renderizar panel
        if self.visible && self.enabled {
            // Renderizar borde
            // Renderizar fondo
            // Renderizar título
            // Renderizar hijos
        }
    }
    
    fn handle_input(&mut self, event: &InputEvent) -> Option<InputAction> {
        // Propagarte input a hijos
        for child in &mut self.children {
            if let Some(action) = child.handle_input(event) {
                return Some(action);
            }
        }
        Some(InputAction::None)
    }
    
    fn set_position(&mut self, pos: UiPos) {
        self.position = pos;
    }
    
    fn set_size(&mut self, size: UiSize) {
        self.size = size;
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    fn is_visible(&self) -> bool {
        self.visible
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn state(&self) -> UiComponentState {
        self.state
    }
}

/// Componente Text
#[derive(Debug, Clone)]
pub struct Text {
    pub id: UiComponentId,
    pub text: String,
    pub position: UiPos,
    pub size: UiSize,
    pub visible: bool,
    pub enabled: bool,
    pub state: UiComponentState,
    pub color: UiColor,
    pub font: UiFont,
    pub align: UiTextAlign,
    pub vertical_align: UiVerticalAlign,
    pub word_wrap: bool,
}

impl Text {
    pub fn new(id: UiComponentId, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            position: UiPos::zero(),
            size: UiSize::new(100.0, 20.0),
            visible: true,
            enabled: true,
            state: UiComponentState::Idle,
            color: UiColor::white(),
            font: UiFont::medium(),
            align: UiTextAlign::Left,
            vertical_align: UiVerticalAlign::Top,
            word_wrap: true,
        }
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
    
    pub fn set_color(&mut self, color: UiColor) {
        self.color = color;
    }
    
    pub fn set_font(&mut self, font: UiFont) {
        self.font = font;
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new(UiComponentId::new(1), "Text")
    }
}

impl UiComponent for Text {
    fn id(&self) -> UiComponentId {
        self.id
    }
    
    fn update(&mut self, _dt: f32) {
        // No requiere actualización
    }
    
    fn render(&self, _renderer: &mut dyn Renderer) {
        // Renderizar texto
    }
    
    fn handle_input(&mut self, _event: &InputEvent) -> Option<InputAction> {
        Some(InputAction::None)
    }
    
    fn set_position(&mut self, pos: UiPos) {
        self.position = pos;
    }
    
    fn set_size(&mut self, size: UiSize) {
        self.size = size;
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    fn is_visible(&self) -> bool {
        self.visible
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn state(&self) -> UiComponentState {
        self.state
    }
}

/// Componente Slider
#[derive(Debug, Clone)]
pub struct Slider {
    pub id: UiComponentId,
    pub label: Option<String>,
    pub position: UiPos,
    pub size: UiSize,
    pub visible: bool,
    pub enabled: bool,
    pub state: UiComponentState,
    pub min_value: f32,
    pub max_value: f32,
    pub current_value: f32,
    pub step: f32,
    pub background_color: UiColor,
    pub handle_color: UiColor,
}

impl Slider {
    pub fn new(id: UiComponentId, label: Option<&str>, min: f32, max: f32) -> Self {
        Self {
            id,
            label: label.map(|s| s.to_string()),
            position: UiPos::zero(),
            size: UiSize::new(200.0, 20.0),
            visible: true,
            enabled: true,
            state: UiComponentState::Idle,
            min_value: min,
            max_value: max,
            current_value: min,
            step: (max - min) / 100.0,
            background_color: UiColor::new(0.3, 0.3, 0.3, 1.0),
            handle_color: UiColor::new(0.6, 0.6, 0.6, 1.0),
        }
    }
    
    pub fn set_value(&mut self, value: f32) {
        self.current_value = value.clamp(self.min_value, self.max_value);
    }
    
    pub fn get_value(&self) -> f32 {
        self.current_value
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new(UiComponentId::new(1), None, 0.0, 100.0)
    }
}

impl UiComponent for Slider {
    fn id(&self) -> UiComponentId {
        self.id
    }
    
    fn update(&mut self, _dt: f32) {
        // No requiere actualización
    }
    
    fn render(&self, _renderer: &mut dyn Renderer) {
        // Renderizar slider
    }
    
    fn handle_input(&mut self, event: &InputEvent) -> Option<InputAction> {
        if !self.visible || !self.enabled {
            return None;
        }
        
        match event {
            InputEvent::MouseMove { position, .. } => {
                if self.is_mouse_over(position) {
                    self.state = UiComponentState::Hovered;
                    let new_value = self.min_value + 
                        ((position.x - self.position.x) / self.size.width) * 
                        (self.max_value - self.min_value);
                    self.current_value = new_value;
                    Some(InputAction::ValueChanged)
                } else {
                    self.state = UiComponentState::Idle;
                    Some(InputAction::None)
                }
            }
            InputEvent::MouseButtonPressed { button, .. } => {
                if *button == MouseButton::Left && self.state == UiComponentState::Hovered {
                    self.state = UiComponentState::Pressed;
                    Some(InputAction::ValueChanged)
                } else {
                    Some(InputAction::None)
                }
            }
            _ => Some(InputAction::None),
        }
    }
    
    fn set_position(&mut self, pos: UiPos) {
        self.position = pos;
    }
    
    fn set_size(&mut self, size: UiSize) {
        self.size = size;
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    fn is_visible(&self) -> bool {
        self.visible
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn state(&self) -> UiComponentState {
        self.state
    }
}

/// Input Event
#[derive(Debug, Clone)]
pub enum InputEvent {
    MouseMove {
        position: UiPos,
        delta: UiPos,
        buttons: Vec<MouseButton>,
    },
    MouseButtonPressed {
        button: MouseButton,
        position: UiPos,
    },
    MouseReleased {
        button: MouseButton,
        position: UiPos,
    },
    MouseWheel {
        delta_y: f32,
        position: UiPos,
    },
    KeyPressed {
        key: KeyCode,
    },
    KeyReleased {
        key: KeyCode,
    },
    FocusGained,
    FocusLost,
}

/// Mouse Button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Keyboard Key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    Enter, Escape, Space, Tab, Backspace, Delete,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
}

/// Acción de input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    None,
    Clicked,
    ValueChanged,
    Typed,
}
