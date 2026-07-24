use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipo de bus de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BusType {
    /// Bus principal (master)
    Master,
    /// Bus de efectos
    Effects,
    /// Bus de música
    Music,
    /// Bus de sonidos
    Sfx,
    /// Bus de voz
    Voice,
    /// Bus de ambiente
    Ambient,
}

impl Default for BusType {
    fn default() -> Self {
        Self::Master
    }
}

/// Bus de audio
#[derive(Debug, Clone)]
pub struct AudioBus {
    pub id: String,
    pub name: String,
    pub bus_type: BusType,
    pub volume: f32,
    pub pan: f32,
    pub muted: bool,
    pub solo: bool,
    pub enabled: bool,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub effects: Vec<String>,
}

impl AudioBus {
    pub fn new(id: String, name: String, bus_type: BusType) -> Self {
        Self {
            id,
            name,
            bus_type,
            volume: 1.0,
            pan: 0.0,
            muted: false,
            solo: false,
            enabled: true,
            inputs: Vec::new(),
            outputs: Vec::new(),
            effects: Vec::new(),
        }
    }

    pub fn add_input(&mut self, input_id: String) {
        if !self.inputs.contains(&input_id) {
            self.inputs.push(input_id);
        }
    }

    pub fn add_output(&mut self, output_id: String) {
        if !self.outputs.contains(&output_id) {
            self.outputs.push(output_id);
        }
    }

    pub fn add_effect(&mut self, effect_id: String) {
        if !self.effects.contains(&effect_id) {
            self.effects.push(effect_id);
        }
    }

    pub fn remove_input(&mut self, input_id: &str) {
        self.inputs.retain(|id| id != input_id);
    }

    pub fn remove_output(&mut self, output_id: &str) {
        self.outputs.retain(|id| id != output_id);
    }

    pub fn remove_effect(&mut self, effect_id: &str) {
        self.effects.retain(|id| id != effect_id);
    }

    pub fn connect(&mut self, target_bus_id: String) {
        self.outputs.push(target_bus_id);
    }

    pub fn is_connected_to(&self, target_bus_id: &str) -> bool {
        self.outputs.contains(&target_bus_id.to_string())
    }

    pub fn get_input_count(&self) -> usize {
        self.inputs.len()
    }

    pub fn get_output_count(&self) -> usize {
        self.outputs.len()
    }

    pub fn get_effect_count(&self) -> usize {
        self.effects.len()
    }
}

/// Sistema de buses de audio
#[derive(Debug, Clone)]
pub struct AudioBusSystem {
    pub buses: HashMap<String, AudioBus>,
    pub master_bus: String,
}

impl AudioBusSystem {
    pub fn new() -> Self {
        let id = "master_bus".to_string();
        let bus = AudioBus::new(id.clone(), "Master Bus".to_string(), BusType::Master);
        let mut temp = Self {
            buses: HashMap::new(),
            master_bus: String::new(),
        };
        Self::add_internal_bus(&mut temp, id.clone(), bus);
        Self {
            buses: temp.buses,
            master_bus: id,
        }
    }

    fn create_master_bus() -> String {
        let id = "master_bus".to_string();
        let bus = AudioBus::new(
            id.clone(),
            "Master Bus".to_string(),
            BusType::Master,
        );
        let mut temp = Self {
            buses: HashMap::new(),
            master_bus: String::new(),
        };
        AudioBusSystem::add_internal_bus(&mut temp, id.clone(), bus);
        id
    }

    fn add_internal_bus(system: &mut Self, id: String, bus: AudioBus) {
        system.buses.insert(id, bus);
    }

    pub fn add_bus(&mut self, bus: AudioBus) {
        self.buses.insert(bus.id.clone(), bus);
    }

    pub fn get_bus(&self, id: &str) -> Option<&AudioBus> {
        self.buses.get(id)
    }

    pub fn get_bus_mut(&mut self, id: &str) -> Option<&mut AudioBus> {
        self.buses.get_mut(id)
    }

    pub fn remove_bus(&mut self, id: &str) {
        self.buses.remove(id);
    }

    pub fn get_master_bus_id(&self) -> &str {
        &self.master_bus
    }

    pub fn connect_bus(&mut self, source_id: &str, target_id: &str) {
        if let Some(source) = self.buses.get_mut(source_id) {
            source.connect(target_id.to_string());
        }
    }

    pub fn get_bus_type(&self, id: &str) -> Option<BusType> {
        self.buses.get(id).map(|b| b.bus_type)
    }

    pub fn get_all_bus_ids(&self) -> Vec<String> {
        self.buses.keys().cloned().collect()
    }

    pub fn get_bus_count(&self) -> usize {
        self.buses.len()
    }
}

impl Default for AudioBusSystem {
    fn default() -> Self {
        Self::new()
    }
}
