use crate::dialogue_ui::{
    DialogueLineType as DialogueLine, DialogueChoice, DialogueVariable, DialogueValue,
    DialogueContext, DialogueManager, DialogueId,
};
use std::collections::HashMap;

/// Estilo de diálogo
#[derive(Debug, Clone, Copy, Default)]
pub struct DialogueStyle {
    pub text_color: crate::ui_system::UiColor,
    pub speaker_color: crate::ui_system::UiColor,
    pub emote_color: crate::ui_system::UiColor,
    pub system_color: crate::ui_system::UiColor,
    pub choice_color: crate::ui_system::UiColor,
    pub background_color: crate::ui_system::UiColor,
    pub border_color: crate::ui_system::UiColor,
    pub font_size: f32,
    pub line_height: f32,
    pub padding_top: f32,
    pub padding_bottom: f32,
    pub padding_left: f32,
    pub padding_right: f32,
}

impl DialogueStyle {
    pub fn default() -> Self {
        Self {
            text_color: crate::ui_system::UiColor::white(),
            speaker_color: crate::ui_system::UiColor::new(0.8, 0.8, 1.0, 1.0),
            emote_color: crate::ui_system::UiColor::new(0.5, 0.5, 0.5, 1.0),
            system_color: crate::ui_system::UiColor::new(0.3, 0.3, 0.3, 1.0),
            choice_color: crate::ui_system::UiColor::new(0.4, 0.4, 0.8, 1.0),
            background_color: crate::ui_system::UiColor::new(0.1, 0.1, 0.15, 0.95),
            border_color: crate::ui_system::UiColor::new(0.3, 0.3, 0.3, 1.0),
            font_size: 16.0,
            line_height: 24.0,
            padding_top: 20.0,
            padding_bottom: 10.0,
            padding_left: 20.0,
            padding_right: 20.0,
        }
    }
}

/// Estado de la UI de diálogo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogueState {
    Hidden,
    Showing,
    Visible,
    Closing,
}

impl Default for DialogueState {
    fn default() -> Self {
        Self::Hidden
    }
}

/// UI de diálogos en juego
#[derive(Debug, Default)]
pub struct DialogueUI {
    pub manager: DialogueManager,
    pub style: DialogueStyle,
    pub state: DialogueState,
    pub is_fade_in: bool,
    pub is_fade_out: bool,
    pub fade_progress: f32,
    pub animation_duration: f32,
    pub current_choice_index: Option<usize>,
    pub is_typing: bool,
    pub typing_progress: f32,
    pub typing_duration: f32,
    pub created_at: std::time::Instant,
    pub last_updated: std::time::Instant,
}

impl DialogueUI {
    /// Crear nueva UI de diálogos
    pub fn new() -> Self {
        Self {
            manager: DialogueManager::new(),
            style: DialogueStyle::default(),
            state: DialogueState::Hidden,
            is_fade_in: false,
            is_fade_out: false,
            fade_progress: 0.0,
            animation_duration: 0.5,
            current_choice_index: None,
            is_typing: false,
            typing_progress: 0.0,
            typing_duration: 1.0,
            created_at: std::time::Instant::now(),
            last_updated: std::time::Instant::now(),
        }
    }
    
    /// Inicializar sistema de diálogos
    pub fn init(&mut self) {
        println!("💬 Dialogue UI initialized");
    }
    
    /// Mostrar diálogo
    pub fn show_dialogue(&mut self, dialogue_id: DialogueId) -> bool {
        let success = self.manager.show_dialogue(dialogue_id);
        if success {
            self.state = DialogueState::Showing;
            self.is_fade_in = true;
            self.fade_progress = 0.0;
            self.last_updated = std::time::Instant::now();
        }
        success
    }
    
    /// Ocultar diálogo
    pub fn hide_dialogue(&mut self) {
        self.manager.hide_dialogue();
        self.state = DialogueState::Closing;
        self.is_fade_out = true;
        self.fade_progress = 0.0;
        self.last_updated = std::time::Instant::now();
    }
    
    /// Siguiente línea
    pub fn next_line(&mut self) {
        self.manager.next_line();
        self.last_updated = std::time::Instant::now();
    }
    
    /// Línea anterior
    pub fn prev_line(&mut self) {
        self.manager.prev_line();
        self.last_updated = std::time::Instant::now();
    }
    
    /// Seleccionar opción
    pub fn select_choice(&mut self, choice_index: usize) {
        self.manager.select_choice(choice_index);
        self.last_updated = std::time::Instant::now();
    }
    
    /// Cerrar diálogo
    pub fn close_dialogue(&mut self) {
        self.manager.close_dialogue();
        self.state = DialogueState::Hidden;
        self.is_fade_out = true;
        self.fade_progress = 0.0;
    }
    
    /// Actualizar UI
    pub fn update(&mut self, dt: f32) {
        self.last_updated = std::time::Instant::now();
        
        // Actualizar fade in
        if self.is_fade_in {
            self.fade_progress += dt / self.animation_duration;
            if self.fade_progress >= 1.0 {
                self.fade_progress = 1.0;
                self.is_fade_in = false;
                self.state = DialogueState::Visible;
            }
        }
        
        // Actualizar fade out
        if self.is_fade_out {
            self.fade_progress += dt / self.animation_duration;
            if self.fade_progress >= 1.0 {
                self.fade_progress = 1.0;
                self.is_fade_out = false;
                self.state = DialogueState::Hidden;
            }
        }
        
        // Actualizar typing
        if self.is_typing {
            self.typing_progress += dt / self.typing_duration;
            if self.typing_progress >= 1.0 {
                self.typing_progress = 1.0;
                self.is_typing = false;
            }
        }
    }
    
    /// Renderizar UI de diálogo
    pub fn render(&mut self, renderer: &mut dyn crate::ui_system::Renderer) {
        if self.state == DialogueState::Hidden || self.manager.context_count() == 0 {
            return;
        }
        
        let alpha = if self.is_fade_in {
            self.fade_progress
        } else if self.is_fade_out {
            1.0 - self.fade_progress
        } else {
            1.0
        };
        
        // Obtener contexto activo
        let context = self.manager.context_manager.active_context().unwrap();
        
        // Renderizar fondo
        self.render_background(renderer, alpha);
        
        // Renderizar líneas de diálogo
        let current_line = context.current_line();
        if let Some(line) = current_line {
            self.render_line(renderer, line, alpha);
        }
        
        // Renderizar opciones
        if !context.choices.is_empty() {
            self.render_choices(renderer, context, alpha);
        }
    }
    
    /// Renderizar fondo del diálogo
    fn render_background(&self, renderer: &mut dyn crate::ui_system::Renderer, alpha: f32) {
        let background = self.style.background_color;
        let width = 800.0;
        let height = 400.0;
        
        renderer.draw(
            0.0,
            360.0,
            background,
            width,
            height,
        );
    }
    
    /// Renderizar línea de diálogo
    fn render_line(&self, renderer: &mut dyn crate::ui_system::Renderer, line: &DialogueLine, alpha: f32) {
        let text_color = if line.is_emote {
            self.style.emote_color
        } else if line.is_system {
            self.style.system_color
        } else if let Some(speaker) = &line.speaker {
            self.style.speaker_color
        } else {
            self.style.text_color
        };
        
        let text = if line.is_emote {
            format!("*{}*", line.text)
        } else {
            line.text.clone()
        };
        
        renderer.draw_text(
            20.0,
            200.0,
            &text,
            text_color,
        );
    }
    
    /// Renderizar opciones
    fn render_choices(&self, renderer: &mut dyn crate::ui_system::Renderer, context: &DialogueContext, alpha: f32) {
        let choices_y = 320.0;
        let choice_spacing = 30.0;
        
        for (index, choice) in context.choices.iter().enumerate() {
            let y = choices_y + (index as f32) * choice_spacing;
            
            let mut text = choice.text.clone();
            if let Some(dialogue_id) = &choice.dialogue_id {
                text = format!("{} ({})", text, dialogue_id.0);
            }
            
            renderer.draw_text(
                20.0,
                y,
                &text,
                self.style.choice_color,
            );
        }
    }
    
    /// Obtener estado actual
    pub fn state(&self) -> DialogueState {
        self.state
    }
    
    /// Verificar si diálogo está visible
    pub fn is_visible(&self) -> bool {
        self.state == DialogueState::Visible || self.state == DialogueState::Showing
    }
    
    /// Verificar si hay líneas restantes
    pub fn has_more_lines(&self) -> bool {
        self.manager.has_more_lines()
    }
    
    /// Obtener línea actual
    pub fn current_line(&self) -> Option<&DialogueLine> {
        self.manager.current_line()
    }
    
    /// Obtener opciones
    pub fn choices(&self) -> Option<&Vec<DialogueChoice>> {
        self.manager.choices()
    }
    
    /// Obtener variable por nombre
    pub fn get_variable(&self, name: &str) -> Option<&DialogueVariable> {
        self.manager.get_variable(name)
    }
    
    /// Obtener valor de variable
    pub fn get_variable_value(&self, name: &str) -> Option<&DialogueValue> {
        self.manager.get_variable_value(name)
    }
    
    /// Verificar si existe variable
    pub fn has_variable(&self, name: &str) -> bool {
        self.manager.has_variable(name)
    }
    
    /// Establecer estilo
    pub fn set_style(&mut self, style: DialogueStyle) {
        self.style = style;
    }
    
    /// Setear duración de animación
    pub fn set_animation_duration(&mut self, duration: f32) {
        self.animation_duration = duration;
    }
    
    /// Setear duración de typing
    pub fn set_typing_duration(&mut self, duration: f32) {
        self.typing_duration = duration;
    }
    
    /// Setear modo de typing
    pub fn set_typing(&mut self, typing: bool) {
        self.is_typing = typing;
    }
    
    /// Tiempo de vida en segundos
    pub fn age(&self) -> f32 {
        self.created_at.elapsed().as_secs_f32()
    }
}

/// Extensiones para PlaySession
pub trait PlaySessionDialogueExt {
    fn show_dialogue(&mut self, dialogue_id: DialogueId);
    fn hide_dialogue(&mut self);
    fn next_dialogue_line(&mut self);
    fn select_dialogue_choice(&mut self, choice_index: usize);
}

impl PlaySessionDialogueExt for crate::play_session::PlaySession {
    fn show_dialogue(&mut self, dialogue_id: DialogueId) {
        self.dialogue_ui.show_dialogue(dialogue_id);
    }
    
    fn hide_dialogue(&mut self) {
        self.dialogue_ui.hide_dialogue();
    }
    
    fn next_dialogue_line(&mut self) {
        self.dialogue_ui.next_line();
    }
    
    fn select_dialogue_choice(&mut self, choice_index: usize) {
        self.dialogue_ui.select_choice(choice_index);
    }
}
