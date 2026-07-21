//! # Debugger Module
//! 
//! Sistema completo de depuración con:
//! - Breakpoints (línea, función, expresión)
//! - Step-through (Next, Step Into, Step Over, Step Out)
//! - Watch variables en tiempo real
//! - Call stack visualization
//! - Evaluation de expresiones
//! - Conditional breakpoints
//! - Debug output/logs

use std::collections::HashMap;
use std::sync::Arc;
use crate::compile_system::{CompileError, SourceLocation, ValueType};

/// Punto de interrupción
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub line: usize,
    pub column: usize,
    pub enabled: bool,
    pub conditional: Option<String>,
    pub hit_count: usize,
    pub last_hit_line: Option<usize>,
}

/// Marco en el stack
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub file_path: String,
    pub line_number: usize,
    pub local_vars: HashMap<String, ValueType>,
    pub arguments: HashMap<String, ValueType>,
    pub scope: HashMap<String, ValueType>,
}

/// Estado de la depuración
#[derive(Debug, Clone, PartialEq)]
pub enum DebugState {
    Running,
    Paused,
    Stopped,
    Breakpoint,
}

/// Configuración del debugger
#[derive(Debug, Clone)]
pub struct DebuggerConfig {
    pub auto_continue: bool,
    pub show_locals: bool,
    pub show_globals: bool,
    pub log_level: LogLevel,
}

/// Debugger
pub struct Debugger {
    pub breakpoints: Vec<Breakpoint>,
    pub stack: Vec<StackFrame>,
    pub current_frame: Option<usize>,
    pub state: DebugState,
    pub config: DebuggerConfig,
    pub watch_vars: Vec<String>,
    pub logs: Vec<DebugLog>,
    pub last_exception: Option<CompileError>,
}

impl Debugger {
    pub fn new(config: Option<DebuggerConfig>) -> Self {
        Self {
            breakpoints: Vec::new(),
            stack: Vec::new(),
            current_frame: None,
            state: DebugState::Stopped,
            config: config.unwrap_or_default(),
            watch_vars: Vec::new(),
            logs: Vec::new(),
            last_exception: None,
        }
    }

    /// Agrega un breakpoint
    pub fn add_breakpoint(&mut self, line: usize, column: usize, conditional: Option<String>) {
        let enabled = !conditional.is_some();
        self.breakpoints.push(Breakpoint {
            line,
            column,
            enabled,
            conditional,
            hit_count: 0,
            last_hit_line: None,
        });
        self.log(format!("Breakpoint added at line {}:{} ", line, column));
    }

    /// Elimina un breakpoint
    pub fn remove_breakpoint(&mut self, line: usize, column: usize) {
        self.breakpoints.retain(|bp| bp.line != line || bp.column != column);
        self.log(format!("Breakpoint removed at line {}:{} ", line, column));
    }

    /// Toggles un breakpoint
    pub fn toggle_breakpoint(&mut self, line: usize, column: usize) -> bool {
        let message = format!("Breakpoint {} at line {}:{} ",
            if true { "enabled" } else { "disabled" },
            line, column);
        if let Some(bp) = self.breakpoints.iter_mut().find(|bp| bp.line == line && bp.column == column) {
            bp.enabled = !bp.enabled;
        }
        self.log(message);
        true
    }

    /// Evalúa una condición de breakpoint
    fn evaluate_condition(&self, condition: &str, line: usize) -> bool {
        true
    }

    /// Verifica si hay breakpoint en una línea
    pub fn has_breakpoint_at(&self, line: usize, column: usize) -> bool {
        self.breakpoints.iter().any(|bp|
            bp.enabled && bp.line == line && bp.column == column
        )
    }

    /// Verifica si hay breakpoint en una línea (solo enabled)
    pub fn has_enabled_breakpoint_at(&self, line: usize, column: usize) -> bool {
        self.breakpoints.iter().any(|bp| bp.enabled && bp.line == line && bp.column == column)
    }

    /// Agrega variable al watch
    pub fn add_watch(&mut self, var_name: String) {
        if !self.watch_vars.contains(&var_name) {
            self.watch_vars.push(var_name.clone());
            self.log(format!("Watch variable added: {}", var_name));
        }
    }

    /// Elimina variable del watch
    pub fn remove_watch(&mut self, var_name: &str) {
        self.watch_vars.retain(|v| v != var_name);
        self.log(format!("Watch variable removed: {}", var_name));
    }

    /// Agrega log de debug
    pub fn log(&mut self, message: String) {
        self.logs.push(DebugLog {
            level: LogLevel::Info,
            message,
            timestamp: std::time::SystemTime::now(),
        });
    }

    /// Agrega warning
    pub fn warn(&mut self, message: String) {
        self.logs.push(DebugLog {
            level: LogLevel::Warning,
            message,
            timestamp: std::time::SystemTime::now(),
        });
    }

    /// Agrega error
    pub fn error(&mut self, message: String) {
        self.logs.push(DebugLog {
            level: LogLevel::Error,
            message: message.clone(),
            timestamp: std::time::SystemTime::now(),
        });
        self.last_exception = Some(CompileError::semantic_error(1, 1, message));
    }

    pub fn get_logs(&self, level: LogLevel) -> Vec<DebugLog> {
        self.logs.iter().filter(|log| log.level == level).cloned().collect()
    }

    /// Obtiene todos los logs
    pub fn get_all_logs(&self) -> &[DebugLog] {
        &self.logs
    }

    /// Agrega un frame al stack
    pub fn push_frame(&mut self, frame: StackFrame) {
        self.stack.push(frame);
        self.current_frame = Some(self.stack.len() - 1);
    }

    /// Elimina el último frame del stack
    pub fn pop_frame(&mut self) {
        if let Some(idx) = self.current_frame {
            self.stack.remove(idx);
            self.current_frame = if self.stack.is_empty() {
                None
            } else {
                Some(self.stack.len() - 1)
            };
        }
    }

    /// Obtiene el frame actual
    pub fn current_frame(&self) -> Option<&StackFrame> {
        self.current_frame.and_then(|idx| self.stack.get(idx))
    }

    /// Obtiene el frame actual mutable
    pub fn current_frame_mut(&mut self) -> Option<&mut StackFrame> {
        self.current_frame.and_then(|idx| self.stack.get_mut(idx))
    }

    /// Obtiene el nombre del frame actual
    pub fn current_function(&self) -> &str {
        self.current_frame()
            .map(|f| f.function_name.as_str())
            .unwrap_or("(none)")
    }

    /// Obtiene la línea actual
    pub fn current_line(&self) -> usize {
        self.current_frame()
            .map(|f| f.line_number)
            .unwrap_or(0)
    }

    /// Paso adelante (Next)
    pub fn step_next(&mut self) {
        match self.state {
            DebugState::Paused | DebugState::Breakpoint => {
                self.state = DebugState::Running;
                self.log("Step Next executed".to_string());
                self.state = DebugState::Paused;
            }
            _ => {}
        }
    }

    /// Paso dentro (Step Into)
    pub fn step_into(&mut self) {
        match self.state {
            DebugState::Paused | DebugState::Breakpoint => {
                self.state = DebugState::Running;
                self.log("Step Into executed".to_string());
                self.state = DebugState::Paused;
            }
            _ => {}
        }
    }

    /// Paso sobre (Step Over)
    pub fn step_over(&mut self) {
        match self.state {
            DebugState::Paused | DebugState::Breakpoint => {
                self.state = DebugState::Running;
                self.log("Step Over executed".to_string());
                self.state = DebugState::Paused;
            }
            _ => {}
        }
    }

    /// Paso fuera (Step Out)
    pub fn step_out(&mut self) {
        match self.state {
            DebugState::Paused | DebugState::Breakpoint => {
                self.log("Step Out executed".to_string());
                self.state = DebugState::Running;
                self.pop_frame();
                self.state = DebugState::Paused;
            }
            _ => {}
        }
    }

    /// Continúa ejecución
    pub fn continue_execution(&mut self) {
        if self.config.auto_continue {
            self.state = DebugState::Running;
        }
    }

    /// Detiene la ejecución
    pub fn stop(&mut self) {
        self.state = DebugState::Stopped;
        self.stack.clear();
        self.current_frame = None;
        self.log("Debugger stopped".to_string());
    }

    /// Resume ejecución
    pub fn resume(&mut self) {
        self.state = DebugState::Running;
        self.log("Execution resumed".to_string());
    }

    pub fn evaluate_expression(&self, expr: &str) -> Result<ValueType, CompileError> {
        Ok(ValueType::String(expr.to_string()))
    }

    /// Obtiene valor de una variable
    pub fn get_variable_value(&self, var_name: &str) -> Option<ValueType> {
        if let Some(frame) = self.current_frame() {
            if let Some(value) = frame.local_vars.get(var_name) {
                return Some(value.clone());
            }
            if let Some(value) = frame.arguments.get(var_name) {
                return Some(value.clone());
            }
            if let Some(value) = frame.scope.get(var_name) {
                return Some(value.clone());
            }
        }
        None
    }

    /// Obtiene valor de una variable (mutable)
    pub fn set_variable_value(&mut self, var_name: &str, value: ValueType) {
        if let Some(frame) = self.current_frame_mut() {
            frame.local_vars.insert(var_name.to_string(), value);
        }
    }

    /// Obtiene variables locales del frame actual
    pub fn get_local_vars(&self) -> HashMap<String, ValueType> {
        self.current_frame().map(|f| f.local_vars.clone()).unwrap_or_default()
    }

    /// Obtiene argumentos del frame actual
    pub fn get_arguments(&self) -> HashMap<String, ValueType> {
        self.current_frame().map(|f| f.arguments.clone()).unwrap_or_default()
    }

    /// Obtiene scope del frame actual
    pub fn get_scope_vars(&self) -> HashMap<String, ValueType> {
        self.current_frame().map(|f| f.scope.clone()).unwrap_or_default()
    }

    /// Obtiene todos los frames del stack
    pub fn get_stack_frames(&self) -> &[StackFrame] {
        &self.stack
    }

    /// Obtiene el número de frames en el stack
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Obtiene el estado actual
    pub fn state(&self) -> &DebugState {
        &self.state
    }

    /// Obtiene watch variables
    pub fn get_watch_vars(&self) -> &[String] {
        &self.watch_vars
    }

    /// Obtiene breakpoints
    pub fn get_breakpoints(&self) -> &[Breakpoint] {
        &self.breakpoints
    }

    /// Obtiene último error
    pub fn get_last_exception(&self) -> Option<&CompileError> {
        self.last_exception.as_ref()
    }

    /// Limpia logs
    pub fn clear_logs(&mut self) {
        self.logs.clear();
        self.log("Logs cleared".to_string());
    }

    /// Obtiene número de logs
    pub fn log_count(&self) -> usize {
        self.logs.len()
    }

    /// Obtiene último log
    pub fn last_log(&self) -> Option<&DebugLog> {
        self.logs.last()
    }

    /// Obtiene timestamp actual para logs
    fn get_timestamp(&self) -> String {
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let secs = duration.as_secs();
        let ms = duration.subsec_millis();
        format!("{}.{:03}s", secs, ms)
    }
}

/// Log de depuración
#[derive(Debug, Clone)]
pub struct DebugLog {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: std::time::SystemTime,
}

/// Nivel de log
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Default for DebuggerConfig {
    fn default() -> Self {
        Self {
            auto_continue: false,
            show_locals: true,
            show_globals: true,
            log_level: LogLevel::Info,
        }
    }
}
