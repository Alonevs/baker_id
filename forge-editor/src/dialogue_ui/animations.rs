use crate::dialogue_ui::DialogueUI;
use crate::audio_system::AudioManager;
use std::time::Instant;

/// Tipos de animación de diálogo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogueAnimationType {
    FadeIn,
    FadeOut,
    SlideIn,
    SlideOut,
    ScaleIn,
    ScaleOut,
    Typing,
}

/// Estado de animación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AnimationState {
    pub is_animating: bool,
    pub progress: f32,
    pub animation_type: DialogueAnimationType,
    pub start_time: Instant,
    pub duration: f32,
    pub ease_function: EaseFunction,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            is_animating: false,
            progress: 0.0,
            animation_type: DialogueAnimationType::FadeIn,
            start_time: Instant::now(),
            duration: 0.5,
            ease_function: EaseFunction::Linear,
        }
    }
    
    pub fn start(&mut self, animation_type: DialogueAnimationType, duration: f32) {
        self.is_animating = true;
        self.animation_type = animation_type;
        self.duration = duration;
        self.start_time = Instant::now();
        self.progress = 0.0;
    }
    
    pub fn update(&mut self, dt: f32) {
        if !self.is_animating {
            return;
        }
        
        self.progress += dt / self.duration;
        
        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.is_animating = false;
        }
    }
    
    pub fn get_progress(&self) -> f32 {
        self.ease_function(self.progress)
    }
    
    pub fn elapsed(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
    
    pub fn is_complete(&self) -> bool {
        !self.is_animating || self.progress >= 1.0
    }
}

/// Funciones de easing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EaseFunction {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
}

impl EaseFunction {
    pub fn linear(t: f32) -> f32 {
        t
    }
    
    pub fn ease_in(t: f32) -> f32 {
        t * t
    }
    
    pub fn ease_out(t: f32) -> f32 {
        t * (2.0 - t)
    }
    
    pub fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }
    
    pub fn ease_in_quad(t: f32) -> f32 {
        t * t
    }
    
    pub fn ease_out_quad(t: f32) -> f32 {
        -t * (t - 2.0)
    }
    
    pub fn ease_in_out_quad(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }
    
    pub fn ease_in_cubic(t: f32) -> f32 {
        t * t * t
    }
    
    pub fn ease_out_cubic(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t)
    }
    
    pub fn ease_in_out_cubic(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0
        }
    }
    
    pub fn ease_in_expo(t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else {
            1.0.f32.powf(2.0 * t)
        }
    }
    
    pub fn ease_out_expo(t: f32) -> f32 {
        if t == 1.0 {
            1.0
        } else {
            1.0 - 1.0.f32.powf(-2.0 * t)
        }
    }
    
    pub fn ease_in_out_expo(t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            0.5 * 1.0.f32.powf(2.0 * t)
        } else {
            1.0 - 0.5 * 1.0.f32.powf(-2.0 * t)
        }
    }
    
    pub fn get(&self, t: f32) -> f32 {
        match self {
            Self::Linear => Self::linear(t),
            Self::EaseIn => Self::ease_in(t),
            Self::EaseOut => Self::ease_out(t),
            Self::EaseInOut => Self::ease_in_out(t),
            Self::EaseInQuad => Self::ease_in_quad(t),
            Self::EaseOutQuad => Self::ease_out_quad(t),
            Self::EaseInOutQuad => Self::ease_in_out_quad(t),
            Self::EaseInCubic => Self::ease_in_cubic(t),
            Self::EaseOutCubic => Self::ease_out_cubic(t),
            Self::EaseInOutCubic => Self::ease_in_out_cubic(t),
            Self::EaseInQuart => t * t * t * t,
            Self::EaseOutQuart => 1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t),
            Self::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - 8.0 * (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t)
                }
            }
            Self::EaseInQuint => t * t * t * t * t,
            Self::EaseOutQuint => 1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t),
            Self::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    1.0 - 16.0 * (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t)
                }
            }
            Self::EaseInExpo => Self::ease_in_expo(t),
            Self::EaseOutExpo => Self::ease_out_expo(t),
            Self::EaseInOutExpo => Self::ease_in_out_expo(t),
            Self::EaseInCirc => {
                let mut t = t;
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - (-t * (t - 1.0)).sqrt()
                }
            }
            Self::EaseOutCirc => {
                let mut t = t;
                if t == 1.0 {
                    1.0
                } else {
                    1.0 + (t - 1.0).sqrt()
                }
            }
            Self::EaseInOutCirc => {
                if t < 0.5 {
                    0.5 * (1.0 - (-2.0 * t).sqrt())
                } else {
                    0.5 * (2.0 + (2.0 * t - 2.0).sqrt())
                }
            }
            Self::EaseInBack => Self::ease_in_back(t),
            Self::EaseOutBack => Self::ease_out_back(t),
            Self::EaseInOutBack => Self::ease_in_out_back(t),
            Self::EaseInElastic => Self::ease_in_elastic(t),
            Self::EaseOutElastic => Self::ease_out_elastic(t),
            Self::EaseInOutElastic => Self::ease_in_out_elastic(t),
        }
    }
    
    fn ease_in_back(t: f32) -> f32 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        t * t * ((c1 + 1.0) * t - c3)
    }
    
    fn ease_out_back(t: f32) -> f32 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        1.0 + c3 * t * t * (t - 1.0)
    }
    
    fn ease_in_out_back(t: f32) -> f32 {
        if t < 0.5 {
            0.5 * Self::ease_in_back(2.0 * t)
        } else {
            0.5 * Self::ease_out_back(2.0 * t - 1.0) + 0.5
        }
    }
    
    fn ease_in_elastic(t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let c4 = (2.0 * std::f32::consts::PI) / 3.0;
            -1.0 * 1.0f32.powf(10.0 * t) * f32::sin((t - 0.05) * c4)
        }
    }
    
    fn ease_out_elastic(t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let c4 = (2.0 * std::f32::consts::PI) / 3.0;
            1.0 * 1.0f32.powf(10.0 * (t - 1.0)) * f32::sin((t - 1.0) * c4) + 1.0
        }
    }
    
    fn ease_in_out_elastic(t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else if t < 0.5 {
            0.5 * Self::ease_in_elastic(2.0 * t)
        } else {
            0.5 * Self::ease_out_elastic(2.0 * t - 1.0) + 0.5
        }
    }
}

/// Sistema de animaciones de diálogo
#[derive(Debug, Default)]
pub struct DialogueAnimationSystem {
    pub fade_in: AnimationState,
    pub fade_out: AnimationState,
    pub slide_in: AnimationState,
    pub slide_out: AnimationState,
    pub scale_in: AnimationState,
    pub scale_out: AnimationState,
    pub typing: AnimationState,
    pub is_playing: bool,
    pub current_animation: DialogueAnimationType,
    pub last_updated: Instant,
}

impl DialogueAnimationSystem {
    pub fn new() -> Self {
        Self {
            fade_in: AnimationState::new(),
            fade_out: AnimationState::new(),
            slide_in: AnimationState::new(),
            slide_out: AnimationState::new(),
            scale_in: AnimationState::new(),
            scale_out: AnimationState::new(),
            typing: AnimationState::new(),
            is_playing: false,
            current_animation: DialogueAnimationType::FadeIn,
            last_updated: Instant::now(),
        }
    }
    
    pub fn init(&mut self) {
        println!("✨ Dialogue Animation System initialized");
    }
    
    pub fn update(&mut self, dt: f32) {
        self.last_updated = Instant::now();
        
        self.fade_in.update(dt);
        self.fade_out.update(dt);
        self.slide_in.update(dt);
        self.slide_out.update(dt);
        self.scale_in.update(dt);
        self.scale_out.update(dt);
        self.typing.update(dt);
        
        self.is_playing = self.fade_in.is_animating || 
                         self.fade_out.is_animating || 
                         self.slide_in.is_animating || 
                         self.slide_out.is_animating || 
                         self.scale_in.is_animating || 
                         self.scale_out.is_animating || 
                         self.typing.is_animating;
    }
    
    pub fn start_fade_in(&mut self, duration: f32) {
        self.fade_in.start(DialogueAnimationType::FadeIn, duration);
    }
    
    pub fn start_fade_out(&mut self, duration: f32) {
        self.fade_out.start(DialogueAnimationType::FadeOut, duration);
    }
    
    pub fn start_typing(&mut self, duration: f32) {
        self.typing.start(DialogueAnimationType::Typing, duration);
    }
    
    pub fn fade_in_progress(&self) -> f32 {
        self.fade_in.get_progress()
    }
    
    pub fn fade_out_progress(&self) -> f32 {
        self.fade_out.get_progress()
    }
    
    pub fn typing_progress(&self) -> f32 {
        self.typing.get_progress()
    }
    
    pub fn is_fade_in_complete(&self) -> bool {
        self.fade_in.is_complete()
    }
    
    pub fn is_fade_out_complete(&self) -> bool {
        self.fade_out.is_complete()
    }
    
    pub fn is_typing_complete(&self) -> bool {
        self.typing.is_complete()
    }
}

/// Extensiones para DialogueUI
pub trait DialogueAnimationExt {
    fn update_animations(&mut self, dt: f32, renderer: &mut dyn crate::ui_system::Renderer);
    fn get_alpha(&self) -> f32;
    fn get_scale(&self) -> f32;
}

impl DialogueAnimationExt for DialogueUI {
    fn update_animations(&mut self, dt: f32, renderer: &mut dyn crate::ui_system::Renderer) {
        let mut animation_system = DialogueAnimationSystem::new();
        animation_system.update(dt);
        
        let alpha = if self.is_fade_in {
            animation_system.fade_in_progress()
        } else if self.is_fade_out {
            1.0 - animation_system.fade_out_progress()
        } else {
            1.0
        };
        
        let scale = 1.0;
        
        self.render_background(renderer, alpha);
    }
    
    fn get_alpha(&self) -> f32 {
        if self.is_fade_in {
            self.fade_progress
        } else if self.is_fade_out {
            1.0 - self.fade_progress
        } else {
            1.0
        }
    }
    
    fn get_scale(&self) -> f32 {
        1.0
    }
}
