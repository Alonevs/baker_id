use std::sync::Arc;
use std::sync::RwLock;

/// Herramientas disponibles en el Toolbar
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolType {
    Select,
    Move,
    Scale,
    Rotate,
    Paint,
    PhysicsBrush,
    TileMap,
    Audio,
    Script,
}

impl Default for ToolType {
    fn default() -> Self {
        Self::Select
    }
}

/// Panel de Toolbar
#[derive(Debug, Clone)]
pub struct Toolbar {
    pub current_tool: ToolType,
    pub tools: Arc<RwLock<Vec<ToolType>>>,
}

impl Default for Toolbar {
    fn default() -> Self {
        let mut tools = Vec::new();
        tools.push(ToolType::Select);
        tools.push(ToolType::Move);
        tools.push(ToolType::Scale);
        tools.push(ToolType::Rotate);
        tools.push(ToolType::Paint);
        tools.push(ToolType::PhysicsBrush);
        tools.push(ToolType::TileMap);
        tools.push(ToolType::Audio);
        tools.push(ToolType::Script);
        
        Self {
            current_tool: ToolType::Select,
            tools: Arc::new(RwLock::new(tools)),
        }
    }
}

impl Toolbar {
    /// Crear nueva Toolbar
    pub fn new() -> Self {
        Self::default()
    }

    /// Obtener herramientas disponibles
    pub fn get_tools(&self) -> Vec<ToolType> {
        self.tools.read().unwrap().clone()
    }

    /// Obtener herramienta actual
    pub fn get_current_tool(&self) -> ToolType {
        self.current_tool
    }

    /// Cambiar herramienta actual
    pub fn set_current_tool(&mut self, tool: ToolType) {
        self.current_tool = tool;
    }

    /// Añadir herramienta
    pub fn add_tool(&mut self, tool: ToolType) {
        self.tools.write().unwrap().push(tool);
    }

    /// Remover herramienta por índice
    pub fn remove_tool(&mut self, index: usize) -> bool {
        self.tools.write().unwrap().remove(index).is_some()
    }

    /// Obtener índice de herramienta
    pub fn tool_index(&self, tool: ToolType) -> Option<usize> {
        self.tools.read().unwrap().iter().position(|t| *t == tool)
    }

    /// Cambiar herramienta por índice
    pub fn set_tool_by_index(&mut self, index: usize) {
        if let Some(tool) = self.tools.read().unwrap().get(index) {
            self.current_tool = *tool;
        }
    }

    /// Activar herramienta de selección
    pub fn select(&mut self) {
        self.current_tool = ToolType::Select;
    }

    /// Activar herramienta de movimiento
    pub fn move_tool(&mut self) {
        self.current_tool = ToolType::Move;
    }

    /// Activar herramienta de escala
    pub fn scale(&mut self) {
        self.current_tool = ToolType::Scale;
    }

    /// Activar herramienta de rotación
    pub fn rotate(&mut self) {
        self.current_tool = ToolType::Rotate;
    }

    /// Activar herramienta de pintura
    pub fn paint(&mut self) {
        self.current_tool = ToolType::Paint;
    }

    /// Activar pincel de físicas
    pub fn physics_brush(&mut self) {
        self.current_tool = ToolType::PhysicsBrush;
    }

    /// Activar herramienta de TileMap
    pub fn tile_map(&mut self) {
        self.current_tool = ToolType::TileMap;
    }

    /// Activar herramienta de audio
    pub fn audio(&mut self) {
        self.current_tool = ToolType::Audio;
    }

    /// Activar herramienta de script
    pub fn script(&mut self) {
        self.current_tool = ToolType::Script;
    }

    /// Exportar configuración a JSON
    pub fn export_config(&self) -> String {
        let tools = self.tools.read().unwrap();
        let current = self.current_tool;
        
        let tools_str: String = tools.iter().map(|t| format!("\"{:?}\"", t)).collect::<Vec<_>>().join(", ");
        
        format!(
            r#"{{
  "current_tool": "{:?}",
  "tools": [{}]
}}"#,
            current,
            tools_str
        )
    }

    /// Importar configuración desde JSON
    pub fn import_config(&mut self, config: &str) {
        // Implementación simple de importación
        // En producción usar serde para parsear JSON completo
    }
}

/// Widget del Toolbar (UI)
pub struct ToolbarWidget {
    pub toolbar: Toolbar,
    pub width: f32,
    pub height: f32,
}

impl Default for ToolbarWidget {
    fn default() -> Self {
        Self {
            toolbar: Toolbar::new(),
            width: 60.0,
            height: 40.0,
        }
    }
}

impl ToolbarWidget {
    /// Crear nuevo widget de Toolbar
    pub fn new() -> Self {
        Self::default()
    }

    /// Renderizar Toolbar con egui
    pub fn render(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar_panel").show(ctx, |ui| {
            ui.heading("Toolbar");
            
            // Renderizar herramientas
            ui.horizontal(|ui| {
                // Botones de herramientas
                let tools = self.toolbar.get_tools();
                let current = self.toolbar.get_current_tool();
                
                for tool in tools {
                    let button_text = match tool {
                        ToolType::Select => "📋 Select",
                        ToolType::Move => "🔄 Move",
                        ToolType::Scale => "⚡ Scale",
                        ToolType::Rotate => "🔃 Rotate",
                        ToolType::Paint => "🎨 Paint",
                        ToolType::PhysicsBrush => "⚛️ Physics",
                        ToolType::TileMap => "🧱 TileMap",
                        ToolType::Audio => "🎵 Audio",
                        ToolType::Script => "💻 Script",
                    };
                    
                    let is_active = tool == current;
                    
                    if ui.selectable_label(is_active, button_text).clicked() {
                        self.toolbar.set_current_tool(tool);
                    }
                }
            });
            
            ui.separator();
            
            // Información de estado
            ui.label(format!("Current Tool: {:?}", current));
        });
    }

    /// Obtener posición del Toolbar
    pub fn position(&self) -> (f32, f32) {
        (0.0, 0.0) // Top-left por defecto
    }

    /// Obtener dimensiones del Toolbar
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}
