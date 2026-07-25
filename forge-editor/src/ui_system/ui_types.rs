use std::collections::HashMap;

/// Identificador único para capas de UI
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiLayerId(pub u64);

impl UiLayerId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Identificador único para componentes UI
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiComponentId(pub u64);

impl UiComponentId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Tipo de componente UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiComponentType {
    Button,
    Panel,
    Text,
    Image,
    Slider,
    InputField,
    ProgressBar,
    ScrollBar,
    Checkbox,
    RadioButton,
    ComboBox,
    ListBox,
    Label,
}

impl Default for UiComponentType {
    fn default() -> Self {
        Self::Button
    }
}

/// Estado de un componente UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiComponentState {
    Idle,
    Hovered,
    Pressed,
    Disabled,
    Selected,
    Focused,
}

impl Default for UiComponentState {
    fn default() -> Self {
        Self::Idle
    }
}

/// Posición en pantalla
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiPos {
    pub x: f32,
    pub y: f32,
}

impl UiPos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Tamaño en pantalla
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiSize {
    pub width: f32,
    pub height: f32,
}

impl UiSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    
    pub fn zero() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}

/// Dimensiones 2D para UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl UiRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn center() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        }
    }
}

/// Color RGBA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl UiColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn white() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
    
    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    
    pub fn red() -> Self {
        Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    
    pub fn green() -> Self {
        Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    }
    
    pub fn blue() -> Self {
        Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }
    }
}

impl Default for UiColor {
    fn default() -> Self {
        Self::white()
    }
}

/// Estilo de fuente
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiFontSize {
    Tiny = 10,
    Small = 12,
    Medium = 16,
    Large = 24,
    Huge = 32,
}

impl Default for UiFontSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Fuente de texto
#[derive(Debug, Clone)]
pub struct UiFont {
    pub font_size: UiFontSize,
    pub font_family: String,
    pub is_bold: bool,
    pub is_italic: bool,
}

impl UiFont {
    pub fn new(size: UiFontSize, family: &str, bold: bool, italic: bool) -> Self {
        Self {
            font_size: size,
            font_family: family.to_string(),
            is_bold,
            is_italic,
        }
    }
    
    pub fn medium() -> Self {
        Self {
            font_size: UiFontSize::Medium,
            font_family: "SansSerif".to_string(),
            is_bold: false,
            is_italic: false,
        }
    }
}

impl Default for UiFont {
    fn default() -> Self {
        Self::medium()
    }
}

/// Tipo de alineación de texto
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiTextAlign {
    Left,
    Center,
    Right,
}

impl Default for UiTextAlign {
    fn default() -> Self {
        Self::Left
    }
}

/// Tipo de alineación vertical
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiVerticalAlign {
    Top,
    Center,
    Bottom,
}

impl Default for UiVerticalAlign {
    fn default() -> Self {
        Self::Top
    }
}

/// Estilo de borde
#[derive(Debug, Clone, Copy)]
pub struct UiBorder {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
    pub color: UiColor,
}

impl UiBorder {
    pub fn new(width: f32, color: UiColor) -> Self {
        Self {
            top: width,
            right: width,
            bottom: width,
            left: width,
            color,
        }
    }
}

/// Estilo de sombra
#[derive(Debug, Clone, Copy)]
pub struct UiShadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub color: UiColor,
}

impl UiShadow {
    pub fn new(offset_x: f32, offset_y: f32, blur: f32, color: UiColor) -> Self {
        Self {
            offset_x,
            offset_y,
            blur_radius: blur,
            color,
        }
    }
}
