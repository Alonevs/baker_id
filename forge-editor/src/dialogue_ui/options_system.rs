use crate::dialogue_ui::DialogueUI;
use std::collections::HashMap;
use serde_json::Value;

/// Opciones de gráficos
#[derive(Debug, Clone, Copy, Default)]
pub struct GraphicsOptions {
    pub resolution: Resolution,
    pub quality: GraphicsQuality,
    pub v_sync: bool,
    pub fullscreen: FullscreenMode,
    pub anti_aliasing: AntiAliasing,
    pub shadows: bool,
    pub particles: bool,
    pub post_processing: bool,
}

impl GraphicsOptions {
    pub fn new() -> Self {
        Self {
            resolution: Resolution::Default,
            quality: GraphicsQuality::Medium,
            v_sync: true,
            fullscreen: FullscreenMode::Windowed,
            anti_aliasing: AntiAliasing::Disabled,
            shadows: true,
            particles: true,
            post_processing: true,
        }
    }
}

/// Resolución
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Default(u32, u32),
    Low(u32, u32),
    Medium(u32, u32),
    High(u32, u32),
    Ultra(u32, u32),
    Custom(u32, u32),
}

impl Default for Resolution {
    fn default() -> Self {
        Self::Default(1920, 1080)
    }
}

impl Resolution {
    pub fn default() -> Self {
        Self::Default(1920, 1080)
    }
    
    pub fn custom(width: u32, height: u32) -> Self {
        Self::Custom(width, height)
    }
    
    pub fn width(&self) -> u32 {
        match self {
            Self::Default(w, _) => *w,
            Self::Low(w, _) => *w,
            Self::Medium(w, _) => *w,
            Self::High(w, _) => *w,
            Self::Ultra(w, _) => *w,
            Self::Custom(w, _) => *w,
        }
    }
    
    pub fn height(&self) -> u32 {
        match self {
            Self::Default(_, h) => *h,
            Self::Low(_, h) => *h,
            Self::Medium(_, h) => *h,
            Self::High(_, h) => *h,
            Self::Ultra(_, h) => *h,
            Self::Custom(_, h) => *h,
        }
    }
    
    pub fn to_u32_pair(&self) -> (u32, u32) {
        match self {
            Self::Default(w, h) => (*w, *h),
            Self::Low(w, h) => (*w, *h),
            Self::Medium(w, h) => (*w, *h),
            Self::High(w, h) => (*w, *h),
            Self::Ultra(w, h) => (*w, *h),
            Self::Custom(w, h) => (*w, *h),
        }
    }
}

/// Calidad gráfica
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphicsQuality {
    #[default]
    Low,
    Medium,
    High,
    Ultra,
}

/// Modo de pantalla completa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FullscreenMode {
    #[default]
    Windowed,
    WindowedFullscreen,
    Exclusive,
}

/// Anti-aliasing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AntiAliasing {
    #[default]
    Disabled,
    X2,
    X4,
    X8,
}

/// Opciones de audio
#[derive(Debug, Clone, Default)]
pub struct AudioOptions {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub dialogue_volume: f32,
    pub ambient_volume: f32,
    pub mute_all: bool,
}

impl AudioOptions {
    pub fn new() -> Self {
        Self {
            master_volume: 0.8,
            sfx_volume: 0.8,
            music_volume: 0.6,
            dialogue_volume: 1.0,
            ambient_volume: 0.5,
            mute_all: false,
        }
    }
    
    /// Aplicar a AudioManager
    pub fn apply_to_audio(&self, manager: &mut crate::audio_system::AudioManager) {
        manager.set_master_volume(self.master_volume);
        manager.set_sfx_volume(self.sfx_volume);
        manager.set_music_volume(self.music_volume);
        manager.set_dialogue_volume(self.dialogue_volume);
        manager.set_ambient_volume(self.ambient_volume);
    }
}

/// Opciones de control
#[derive(Debug, Clone, Default)]
pub struct ControlOptions {
    pub mouse_sensitivity: f32,
    pub keyboard_layout: KeyboardLayout,
    pub inverted_y_axis: bool,
    pub show_cursor: bool,
    pub disable_mouse: bool,
    pub enable_gamepad: bool,
}

impl ControlOptions {
    pub fn new() -> Self {
        Self {
            mouse_sensitivity: 1.0,
            keyboard_layout: KeyboardLayout::US,
            inverted_y_axis: false,
            show_cursor: true,
            disable_mouse: false,
            enable_gamepad: true,
        }
    }
}

/// Diseño de teclado
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyboardLayout {
    #[default]
    US,
    UK,
    DE,
    FR,
    ES,
}

/// Opciones de accesibilidad
#[derive(Debug, Clone, Default)]
pub struct AccessibilityOptions {
    pub subtitles_enabled: bool,
    pub subtitle_size: SubtitleSize,
    pub color_blind_mode: ColorBlindMode,
    pub reduced_motion: bool,
    pub high_contrast: bool,
    pub screen_reader: bool,
}

impl AccessibilityOptions {
    pub fn new() -> Self {
        Self {
            subtitles_enabled: true,
            subtitle_size: SubtitleSize::Medium,
            color_blind_mode: ColorBlindMode::None,
            reduced_motion: false,
            high_contrast: false,
            screen_reader: false,
        }
    }
}

/// Tamaño de subtítulos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubtitleSize {
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
}

/// Modo de daltonismo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorBlindMode {
    #[default]
    None,
    Protanopia,
    Deuteranopia,
    Tritanopia,
    Protanomaly,
    Deuteranomaly,
    Tritanomaly,
}

/// Gestión de opciones del juego
#[derive(Debug, Default)]
pub struct OptionsManager {
    pub graphics: GraphicsOptions,
    pub audio: AudioOptions,
    pub control: ControlOptions,
    pub accessibility: AccessibilityOptions,
    pub saved_at: std::time::Instant,
    pub options: HashMap<String, Value>,
}

impl OptionsManager {
    /// Crear nuevo gestor de opciones
    pub fn new() -> Self {
        Self {
            graphics: GraphicsOptions::default(),
            audio: AudioOptions::new(),
            control: ControlOptions::new(),
            accessibility: AccessibilityOptions::new(),
            saved_at: std::time::Instant::now(),
        }
    }
    
    /// Inicializar opciones
    pub fn init(&mut self) {
        println!("⚙️ Options Manager initialized");
    }
    
    /// Aplicar opciones a AudioManager
    pub fn apply_audio_options(&self, manager: &mut crate::audio_system::AudioManager) {
        self.audio.apply_to_audio(manager);
    }
    
    /// Aplicar opciones a DialogueUI
    pub fn apply_dialogue_options(&mut self, ui: &mut DialogueUI) {
        ui.set_typing(self.accessibility.screen_reader);
    }
    
    /// Guardar opciones
    pub fn save(&mut self) {
        self.saved_at = std::time::Instant::now();
        println!("💾 Options saved at {:?}", self.saved_at);
    }
    
    /// Cargar opciones
    pub fn load(&mut self) {
        println!("📥 Options loaded");
    }
    
    /// Obtener todas las opciones
    pub fn get_all(&self) -> &HashMap<String, Value> {
        &self.options
    }
    
    /// Obtener opciones de gráficos
    pub fn graphics(&self) -> &GraphicsOptions {
        &self.graphics
    }
    
    /// Obtener opciones de audio
    pub fn audio(&self) -> &AudioOptions {
        &self.audio
    }
    
    /// Obtener opciones de control
    pub fn control(&self) -> &ControlOptions {
        &self.control
    }
    
    /// Obtener opciones de accesibilidad
    pub fn accessibility(&self) -> &AccessibilityOptions {
        &self.accessibility
    }
    
    /// Tiempo de vida en segundos
    pub fn age(&self) -> f32 {
        self.saved_at.elapsed().as_secs_f32()
    }
}
