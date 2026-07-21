use std::sync::Arc;
use std::sync::OnceLock;
use crate::bitacora_validator::BitacoraValidator;

static INSTANCE: OnceLock<BitacoraValidatorSingleton> = OnceLock::new();

/// Singleton global para el validador de bitácora
pub struct BitacoraValidatorSingleton {
    validator: Arc<BitacoraValidator>,
}

impl Default for BitacoraValidatorSingleton {
    fn default() -> Self {
        Self {
            validator: Arc::new(BitacoraValidator::new()),
        }
    }
}

impl BitacoraValidatorSingleton {
    /// Obtener instancia única del singleton
    pub fn instance() -> &'static Self {
        INSTANCE.get_or_init(BitacoraValidatorSingleton::default)
    }

    /// Obtener referencia al validador
    pub fn get_validator(&self) -> &Arc<BitacoraValidator> {
        &self.validator
    }

    /// Obtener errores del linter
    pub fn get_errors(&self) -> Vec<String> {
        self.validator.get_errors().iter().map(|m| m.to_string()).collect()
    }

    /// Obtener warnings del linter
    pub fn get_warnings(&self) -> Vec<String> {
        self.validator.get_warnings().iter().map(|m| m.to_string()).collect()
    }

    /// Obtener infos del linter
    pub fn get_infos(&self) -> Vec<String> {
        self.validator.get_infos().iter().map(|m| m.to_string()).collect()
    }

    /// Verificar si hay errores
    pub fn has_errors(&self) -> bool {
        !self.validator.get_errors().is_empty()
    }
}

/// Función para obtener el singleton
pub fn get_validator_singleton() -> &'static BitacoraValidatorSingleton {
    BitacoraValidatorSingleton::instance()
}

