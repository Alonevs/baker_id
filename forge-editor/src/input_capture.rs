//! # Input Capture
//! 
//! Captura input del usuario en tiempo real para Play Mode:
//! - Teclado (WASD, flechas, etc.)
//! - Ratón (posición, clics, scroll)
//! - Soporte para múltiples inputs simultáneos

use std::collections::HashMap;

/// Tecla presionada
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    /// Tecla A
    A,
    /// Tecla B
    B,
    /// Tecla C
    C,
    /// Tecla D
    D,
    /// Tecla E
    E,
    /// Tecla F
    F,
    /// Tecla G
    G,
    /// Tecla H
    H,
    /// Tecla I
    I,
    /// Tecla J
    J,
    /// Tecla K
    K,
    /// Tecla L
    L,
    /// Tecla M
    M,
    /// Tecla N
    N,
    /// Tecla O
    O,
    /// Tecla P
    P,
    /// Tecla Q
    Q,
    /// Tecla R
    R,
    /// Tecla S
    S,
    /// Tecla T
    T,
    /// Tecla U
    U,
    /// Tecla V
    V,
    /// Tecla W
    W,
    /// Tecla X
    X,
    /// Tecla Y
    Y,
    /// Tecla Z
    Z,
    /// Espacio
    Space,
    /// Enter
    Enter,
    /// Escape
    Escape,
    /// Tab
    Tab,
    /// Backspace
    Backspace,
    /// Flecha arriba
    Up,
    /// Flecha abajo
    Down,
    /// Flecha izquierda
    Left,
    /// Flecha derecha
    Right,
    /// Flecha home
    Home,
    /// Flecha end
    End,
    /// Flecha begin
    PageUp,
    /// Flecha page down
    PageDown,
    /// Insertar
    Insert,
    /// Borrar
    Delete,
    /// F1
    F1,
    /// F2
    F2,
    /// F3
    F3,
    /// F4
    F4,
    /// F5
    F5,
    /// F6
    F6,
    /// F7
    F7,
    /// F8
    F8,
    /// F9
    F9,
    /// F10
    F10,
    /// F11
    F11,
    /// F12
    F12,
    /// 0
    Num0,
    /// 1
    Num1,
    /// 2
    Num2,
    /// 3
    Num3,
    /// 4
    Num4,
    /// 5
    Num5,
    /// 6
    Num6,
    /// 7
    Num7,
    /// 8
    Num8,
    /// 9
    Num9,
    /// Coma
    Comma,
    /// Punto
    Minus,
    /// Punto y coma
    Period,
    /// Slash
    Slash,
    /// Open bracket
    OpenBracket,
    /// Close bracket
    CloseBracket,
    /// Quote
    Quote,
    /// Semicolon
    Semicolon,
    /// Backquote
    Backquote,
}

/// Estado del ratón
#[derive(Debug, Clone, Default)]
pub struct MouseState {
    /// Posición del ratón
    pub position: (f32, f32),
    /// Estado del botón izquierdo
    pub left_button: bool,
    /// Estado del botón derecho
    pub right_button: bool,
    /// Estado del botón central
    pub middle_button: bool,
    /// Estado de scroll vertical
    pub scroll_y: f32,
    /// Estado de scroll horizontal
    pub scroll_x: f32,
}

impl MouseState {
    /// Crea un nuevo estado de ratón vacío
    pub fn new() -> Self {
        Self::default()
    }

    /// Restaura el estado por defecto
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

/// Input del usuario capturado
#[derive(Debug, Clone)]
pub struct UserInput {
    /// Estado de las teclas
    pub keyboard: HashMap<KeyCode, bool>,
    /// Estado del ratón
    pub mouse: MouseState,
    /// Tiempo de último input
    pub last_input_time: f64,
}

impl Default for UserInput {
    fn default() -> Self {
        Self {
            keyboard: HashMap::new(),
            mouse: MouseState::new(),
            last_input_time: 0.0,
        }
    }
}

/// Captura de input del usuario
#[derive(Debug)]
pub struct InputCapture {
    /// Estado de las teclas
    pub keyboard: HashMap<KeyCode, bool>,
    /// Estado del ratón
    pub mouse: MouseState,
}

impl InputCapture {
    /// Crea un nuevo capturador de input
    pub fn new() -> Self {
        Self {
            keyboard: HashMap::new(),
            mouse: MouseState::new(),
        }
    }

    /// Actualiza el estado de input
    pub fn update(&mut self, input: &UserInput) {
        // Actualizar teclado
        self.keyboard.clear();
        for (key, pressed) in &input.keyboard {
            self.keyboard.insert(*key, *pressed);
        }

        // Actualizar ratón
        self.mouse.position = input.mouse.position;
        self.mouse.left_button = input.mouse.left_button;
        self.mouse.right_button = input.mouse.right_button;
        self.mouse.middle_button = input.mouse.middle_button;
        self.mouse.scroll_y = input.mouse.scroll_y;
        self.mouse.scroll_x = input.mouse.scroll_x;
    }

    /// Verifica si una tecla está presionada
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        *self.keyboard.get(&key).unwrap_or(&false)
    }

    /// Verifica si una tecla fue presionada en este frame
    pub fn is_key_just_pressed(&self, key: KeyCode, prev_state: &HashMap<KeyCode, bool>) -> bool {
        self.is_key_pressed(key) && !prev_state.get(&key).unwrap_or(&false)
    }

    /// Obtiene el movimiento del jugador (WASD)
    pub fn get_movement(&self) -> (f32, f32) {
        let horizontal = if self.is_key_pressed(KeyCode::D) as i32 - self.is_key_pressed(KeyCode::A) as i32 != 0 {
            self.is_key_pressed(KeyCode::D) as i32 - self.is_key_pressed(KeyCode::A) as i32
        } else {
            0
        };

        let vertical = if self.is_key_pressed(KeyCode::W) as i32 - self.is_key_pressed(KeyCode::S) as i32 != 0 {
            self.is_key_pressed(KeyCode::W) as i32 - self.is_key_pressed(KeyCode::S) as i32
        } else {
            0
        };

        (horizontal as f32, vertical as f32)
    }
}

impl Default for InputCapture {
    fn default() -> Self {
        Self::new()
    }
}
