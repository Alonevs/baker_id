//! # Property Panel API
//! 
//! Módulo para gestionar propiedades de entidades en el editor.

use crate::ui::Widget;

/// Identificador único de una entidad
#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct EntityId(pub i32);

impl From<uuid::Uuid> for EntityId {
    fn from(uuid: uuid::Uuid) -> Self {
        EntityId(uuid.as_u128() as i32)
    }
}

impl EntityId {
    /// Convierte EntityId a Uuid
    pub fn as_uuid(&self) -> uuid::Uuid {
        uuid::Uuid::from_u128(self.0 as u128)
    }
}

/// Propiedades de transformación de una entidad
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformProperties {
    /// Posición en 3D
    pub position: (f32, f32, f32),
    /// Rotación en 3D (grados)
    pub rotation: (f32, f32, f32),
    /// Escala en 3D
    pub scale: (f32, f32, f32),
    /// Estado de visibilidad
    pub visible: bool,
}

impl Default for TransformProperties {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            visible: true,
        }
    }
}

/// Propiedades de componentes de una entidad
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentProperties {
    /// Component type
    pub component_type: String,
    /// Keys
    pub keys: Vec<String>,
    /// Values
    pub values: Vec<String>,
    /// Enabled
    pub enabled: bool,
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            component_type: "None".to_string(),
            keys: Vec::new(),
            values: Vec::new(),
            enabled: true,
        }
    }
}

/// Propiedades de scripts de una entidad
#[derive(Debug, Clone, PartialEq)]
pub struct ScriptProperties {
    /// Script name
    pub script_name: String,
    /// Script path
    pub script_path: String,
    /// Script lines
    pub script_lines: Vec<String>,
    /// Current line
    pub current_line: usize,
    /// Is compiled
    pub is_compiled: bool,
}

impl Default for ScriptProperties {
    fn default() -> Self {
        Self {
            script_name: "Untitled".to_string(),
            script_path: String::new(),
            script_lines: Vec::new(),
            current_line: 0,
            is_compiled: false,
        }
    }
}

/// Panel de propiedades para entidades
#[derive(Debug, Clone)]
pub struct PropertyPanel {
    widgets: Vec<Widget>,
    selected_entity: Option<EntityId>,
    transform_props: Option<TransformProperties>,
    component_props: Option<ComponentProperties>,
    script_props: Option<ScriptProperties>,
    tab: PropertyTab,
}

/// Pestañas del panel de propiedades
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyTab {
    Transform,
    Component,
    Script,
}

impl PropertyPanel {
    /// Crea un nuevo panel de propiedades
    pub fn new() -> Self {
        PropertyPanel {
            widgets: Vec::new(),
            selected_entity: None,
            transform_props: None,
            component_props: None,
            script_props: None,
            tab: PropertyTab::Transform,
        }
    }

    /// Obtiene el ID de la entidad seleccionada
    pub fn get_selected_entity(&self) -> Option<EntityId> {
        self.selected_entity.as_ref().map(|id| *id)
    }

    /// Establece la entidad seleccionada y actualiza sus propiedades
    pub fn set_selected_entity(&mut self, entity_id: EntityId) {
        self.selected_entity = Some(entity_id);
        self.refresh_properties();
    }

    /// Actualiza las propiedades de la entidad seleccionada
    fn refresh_properties(&mut self) {
        if let Some(_entity_id) = self.selected_entity.clone() {
            self.transform_props = Some(TransformProperties {
                position: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0),
                scale: (1.0, 1.0, 1.0),
                visible: true,
            });
            self.component_props = Some(ComponentProperties {
                component_type: "None".to_string(),
                keys: Vec::new(),
                values: Vec::new(),
                enabled: true,
            });
            self.script_props = Some(ScriptProperties {
                script_name: "Untitled".to_string(),
                script_path: String::new(),
                script_lines: Vec::new(),
                current_line: 0,
                is_compiled: false,
            });
        }
    }

    /// Obtiene las propiedades de transformación
    pub fn get_transform_properties(&self) -> Option<&TransformProperties> {
        self.transform_props.as_ref()
    }

    /// Obtiene las propiedades de componentes
    pub fn get_component_properties(&self) -> Option<&ComponentProperties> {
        self.component_props.as_ref()
    }

    /// Obtiene las propiedades de scripts
    pub fn get_script_properties(&self) -> Option<&ScriptProperties> {
        self.script_props.as_ref()
    }

    /// Establece las propiedades de transformación
    pub fn set_transform_props(&mut self, transform: TransformProperties) {
        self.transform_props = Some(transform);
    }

    /// Establece las propiedades de componentes
    pub fn set_component_props(&mut self, component: ComponentProperties) {
        self.component_props = Some(component);
    }

    /// Establece las propiedades de scripts
    pub fn set_script_props(&mut self, script: ScriptProperties) {
        self.script_props = Some(script);
    }

    /// Establece la posición de la entidad
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        if let Some(props) = self.transform_props.as_mut() {
            props.position = (x, y, z);
        }
    }

    /// Establece la rotación de la entidad
    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        if let Some(props) = self.transform_props.as_mut() {
            props.rotation = (x, y, z);
        }
    }

    /// Establece la escala de la entidad
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        if let Some(props) = self.transform_props.as_mut() {
            props.scale = (x, y, z);
        }
    }

    /// Establece la visibilidad de la entidad
    pub fn set_visible(&mut self, visible: bool) {
        if let Some(props) = self.transform_props.as_mut() {
            props.visible = visible;
        }
    }

    /// Agrega un componente a la entidad
    pub fn add_component(&mut self, component_type: &str) {
        if let Some(ref mut props) = self.component_props {
            props.keys.push(component_type.to_string());
        }
    }

    /// Elimina un componente de la entidad
    pub fn remove_component(&mut self, index: usize) {
        if let Some(ref mut props) = self.component_props {
            if index < props.keys.len() {
                props.keys.remove(index);
            }
        }
    }

    /// Agrega un script a la entidad
    pub fn add_script(&mut self, script_type: &str) {
        if let Some(ref mut props) = self.script_props {
            props.script_lines.push(script_type.to_string());
        }
    }

    /// Elimina un script de la entidad
    pub fn remove_script(&mut self, index: usize) {
        if let Some(ref mut props) = self.script_props {
            if index < props.script_lines.len() {
                props.script_lines.remove(index);
            }
        }
    }

    /// Obtiene los widgets del panel
    pub fn get_widgets(&self) -> &Vec<Widget> {
        &self.widgets
    }

    /// Obtiene la pestaña actual
    pub fn get_tab(&self) -> &PropertyTab {
        &self.tab
    }

    /// Establece la pestaña activa
    pub fn set_tab(&mut self, tab: PropertyTab) {
        self.tab = tab;
        self.refresh_properties();
    }
}

