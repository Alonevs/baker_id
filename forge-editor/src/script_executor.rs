//! # Script Executor Module
//! 
//! Sistema de ejecución de scripts que interpreta el AST generado por el compilador.

use crate::compile_system::{ASTNode, CompileError, CompileWarning, DataType, SourceLocation, ValueType, ErrorKind};
use std::collections::HashMap;
use std::sync::Arc;

/// Resultado de compilación
#[derive(Debug, Clone)]
pub struct CompileResult {
    pub success: bool,
    pub ast: Option<crate::compile_system::ASTNode>,
    pub errors: Vec<CompileError>,
    pub warnings: Vec<CompileWarning>,
}

impl CompileResult {
    pub fn new(success: bool, ast: Option<crate::compile_system::ASTNode>) -> Self {
        Self {
            success,
            ast,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn type_mismatch(expected: &str, actual: &str) -> Self {
        Self {
            success: false,
            ast: None,
            errors: vec![CompileError::type_mismatch(1, 1, format!("Mismatched types: expected {}, found {}", expected, actual))],
            warnings: Vec::new(),
        }
    }
}

/// Contexto de ejecución
#[derive(Clone)]
pub struct ExecutionContext {
    pub variables: HashMap<String, ValueType>,
    pub functions: HashMap<String, Arc<dyn Fn(HashMap<String, ValueType>) -> ValueType>>,
    pub scope_stack: Vec<HashMap<String, ValueType>>,
    pub current_scope: Option<String>,
    pub error_location: Option<SourceLocation>,
}

impl std::fmt::Debug for ExecutionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExecutionContext")
            .field("variables", &self.variables)
            .field("functions_count", &self.functions.len())
            .field("scope_stack", &self.scope_stack)
            .field("current_scope", &self.current_scope)
            .field("error_location", &self.error_location)
            .finish()
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            scope_stack: Vec::new(),
            current_scope: None,
            error_location: None,
        }
    }

    pub fn push_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }

    pub fn get_variable(&self, name: &str) -> Option<&ValueType> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        if let Some(value) = self.variables.get(name) {
            return Some(value);
        }
        None
    }

    pub fn set_variable(&mut self, name: &str, value: ValueType) {
        if let Some(last_scope) = self.scope_stack.last_mut() {
            last_scope.insert(name.to_string(), value);
        } else {
            self.variables.insert(name.to_string(), value);
        }
    }

    pub fn register_function(&mut self, name: String, func: Arc<dyn Fn(HashMap<String, ValueType>) -> ValueType>) {
        self.functions.insert(name, func);
    }

    pub fn get_function(&self, name: &str) -> Option<Arc<dyn Fn(HashMap<String, ValueType>) -> ValueType>> {
        self.functions.get(name).cloned()
    }

    pub fn error(&mut self, message: String, location: SourceLocation) {
        self.error_location = Some(location.clone());
        println!("[ERROR] {}: {}", location.line, message);
    }

    pub fn clear_error(&mut self) {
        self.error_location = None;
    }

    pub fn has_error(&self) -> bool {
        self.error_location.is_some()
    }
}

/// Executor de scripts
#[derive(Debug, Clone)]
pub struct ScriptExecutor {
    pub context: ExecutionContext,
    pub execution_count: usize,
    pub total_execution_time: f32,
}

impl ScriptExecutor {
    pub fn new() -> Self {
        Self {
            context: ExecutionContext::new(),
            execution_count: 0,
            total_execution_time: 0.0,
        }
    }

    /// Ejecuta un nodo del AST
    pub fn execute_node(&mut self, node: &ASTNode) -> Result<ValueType, CompileError> {
        match node {
            ASTNode::Literal { value } => Ok(value.clone()),
            
            ASTNode::Identifier { name } => {
                match self.context.get_variable(name.as_str()) {
                    Some(value) => Ok(value.clone()),
                    None => Err(CompileError::semantic_error(
                        1, 1,
                        format!("Variable '{}' no declarada", name)
                    )),
                }
            }

            ASTNode::VariableDeclaration { name, value, data_type } => {
                let computed_value = self.execute_node(value)?;
                let typed_value = if let Some(dt) = data_type {
                    self.type_value(&computed_value, dt)
                } else {
                    computed_value
                };
                self.context.set_variable(name, typed_value.clone());
                Ok(typed_value)
            }

            ASTNode::Assignment { name, value } => {
                let computed_value = self.execute_node(value)?;
                self.context.set_variable(name, computed_value.clone());
                Ok(computed_value)
            }

            ASTNode::BinaryOp { left, operator, right } => {
                let left_value = self.execute_node(left)?;
                let right_value = self.execute_node(right)?;
                self.execute_binary_op(left_value, operator, right_value)
            }

            ASTNode::UnaryOp { operator, operand } => {
                let operand_value = self.execute_node(operand)?;
                self.execute_unary_op(operator, operand_value)
            }

            ASTNode::FunctionCall { name, arguments } => {
                if let Some(func) = self.context.get_function(name.as_str()) {
                    let args = arguments.iter()
                        .map(|arg| self.execute_node(arg))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| e.clone())?;
                    
                    let mut params = HashMap::new();
                    for (i, arg) in args.iter().enumerate() {
                        if i < arguments.len() {
                            let param_name = match &arguments[i] {
                                ASTNode::Identifier { name } => name.clone(),
                                _ => format!("arg{}", i),
                            };
                            params.insert(param_name, arg.clone());
                        }
                    }
                    
                    let result = func(params);
                    self.context.clear_error();
                    Ok(result)
                } else {
                    Err(CompileError::semantic_error(
                        1, 1,
                        format!("Función '{}' no declarada", name)
                    ))
                }
            }

            ASTNode::MethodCall { receiver, method, arguments } => {
                // Implementación básica de llamada a método
                let receiver_value = self.execute_node(receiver)?;
                println!("Method call: {} on {:?}", method, receiver_value);
                Err(CompileError::semantic_error(
                    1, 1,
                    "Métodos no implementados en este nivel".to_string()
                ))
            }

            ASTNode::PropertyAccess { object, property } => {
                // Implementación básica de acceso a propiedad
                let object_value = self.execute_node(object)?;
                println!("Property access: {} on {:?}", property, object_value);
                Err(CompileError::semantic_error(
                    1, 1,
                    "Acceso a propiedades no implementado en este nivel".to_string()
                ))
            }

            ASTNode::Block { statements } => {
                for stmt in statements {
                    self.execute_node(stmt)?;
                }
                Ok(ValueType::Null)
            }

            ASTNode::IfElse { condition, then_branch, else_branch } => {
                let condition_value = self.execute_node(condition)?;
                let condition_bool = self.to_bool(&condition_value);
                
                if condition_bool {
                    for stmt in then_branch {
                        self.execute_node(stmt)?;
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.execute_node(stmt)?;
                    }
                }
                Ok(ValueType::Null)
            }

            ASTNode::WhileLoop { condition, body } => {
                loop {
                    let condition_value = self.execute_node(condition)?;
                    let condition_bool = self.to_bool(&condition_value);
                    
                    if !condition_bool {
                        break;
                    }
                    
                    for stmt in body {
                        self.execute_node(stmt)?;
                    }
                }
                Ok(ValueType::Null)
            }

            ASTNode::ConstDeclaration { name, value, data_type } => {
                let computed_value = self.execute_node(value)?;
                let typed_value = if let Some(dt) = data_type {
                    self.type_value(&computed_value, dt)
                } else {
                    computed_value
                };
                self.context.variables.insert(name.to_string(), typed_value.clone());
                Ok(typed_value)
            }

            ASTNode::Print { value } => {
                let value_to_print = self.execute_node(value)?;
                println!("{}", value_to_print.to_string());
                Ok(ValueType::Null)
            }

            ASTNode::ArrayLiteral { elements } => {
                let mut array = Vec::new();
                for elem in elements {
                    array.push(self.execute_node(elem)?);
                }
                Ok(ValueType::Array(array))
            }

            ASTNode::ObjectLiteral { properties } => {
                let mut object = HashMap::new();
                for (key, value) in properties {
                    object.insert(key.clone(), self.execute_node(value)?);
                }
                Ok(ValueType::Object(object))
            }

            ASTNode::Return { value } => {
                let return_value = if let Some(v) = value {
                    self.execute_node(v)?
                } else {
                    ValueType::Null
                };
                Err(CompileError::semantic_error(
                    1, 1,
                    "Return statement executed (function not implemented)".to_string()
                ))
            }

            ASTNode::ArrayAssignment { index, value } => {
                // Implementación básica de asignación a array
                let index_value = self.execute_node(index)?;
                let value_to_assign = self.execute_node(value)?;
                println!("Array assignment: {} = {:?}", index_value, value_to_assign);
                Err(CompileError::semantic_error(
                    1, 1,
                    "Asignación a arrays no implementada en este nivel".to_string()
                ))
            }

            ASTNode::FunctionDefinition { name, parameters, body, .. } => {
                // Crear función closure
                let params = parameters.clone();
                let body_clone = body.clone();
                let executor_clone = self.clone();
                let name_clone = name.clone();
                
                let func = Arc::new(move |params_map: HashMap<String, ValueType>| -> ValueType {
                    let mut context = ExecutionContext::new();
                    context.scope_stack.push(params_map);
                    context.current_scope = Some(name_clone.clone());
                    
                    let mut local_executor = executor_clone.clone();
                    for stmt in &body_clone {
                        match local_executor.execute_node(stmt) {
                            Ok(_) => {}
                            Err(e) => {
                                context.error(e.message, e.location);
                            }
                        }
                    }
                    
                    context.variables.get("result").cloned()
                        .unwrap_or(ValueType::Null)
                });
                
                self.context.register_function(name.clone(), func);
                Ok(ValueType::Null)
            }

            ASTNode::Logical { left, operator, right } => {
                let left_value = self.execute_node(left)?;
                let left_bool = self.to_bool(&left_value);
                
                match operator {
                    crate::compile_system::LogicalOperator::And => {
                        if !left_bool {
                            return Ok(ValueType::Bool(false));
                        }
                        let right_value = self.execute_node(right)?;
                        Ok(ValueType::Bool(self.to_bool(&right_value)))
                    }
                    crate::compile_system::LogicalOperator::Or => {
                        if left_bool {
                            return Ok(ValueType::Bool(true));
                        }
                        let right_value = self.execute_node(right)?;
                        Ok(ValueType::Bool(self.to_bool(&right_value)))
                    }
                    crate::compile_system::LogicalOperator::Not => {
                        Ok(ValueType::Bool(!left_bool))
                    }
                }
            }

            ASTNode::Arithmetic { left, operator, right } => {
                let left_value = self.execute_node(left)?;
                let right_value = self.execute_node(right)?;
                self.execute_arithmetic_op(left_value, operator, right_value)
            }
        }
    }

    /// Evalúa un valor a booleano
    fn to_bool(&self, value: &ValueType) -> bool {
        match value {
            ValueType::Bool(b) => *b,
            ValueType::Int(i) => *i != 0,
            ValueType::Float(f) => *f != 0.0,
            ValueType::String(s) => !s.is_empty(),
            ValueType::Array(arr) => !arr.is_empty(),
            ValueType::Object(obj) => !obj.is_empty(),
            ValueType::Null => false,
        }
    }

    /// Ejecuta una operación binaria
    fn execute_binary_op(&self, left: ValueType, op: &crate::compile_system::BinaryOperator, right: ValueType) -> Result<ValueType, CompileError> {
        match op {
            crate::compile_system::BinaryOperator::Plus => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => Ok(ValueType::Int(*a + *b)),
                    (ValueType::Float(a), ValueType::Float(b)) => Ok(ValueType::Float(*a + *b)),
                    (ValueType::String(a), ValueType::String(b)) => Ok(ValueType::String(format!("{}{}", a, b))),
                    (ValueType::String(a), ValueType::Int(b)) => Ok(ValueType::String(format!("{}{}", a, b))),
                    (ValueType::Int(a), ValueType::String(b)) => Ok(ValueType::String(format!("{}{}", a, b))),
                    (ValueType::Int(a), ValueType::Float(b)) => Ok(ValueType::Float(*a as f64 + *b)),
                    (ValueType::Float(a), ValueType::Int(b)) => Ok(ValueType::Float(*a + *b as f64)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación + inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::Minus => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => Ok(ValueType::Int(*a - *b)),
                    (ValueType::Float(a), ValueType::Float(b)) => Ok(ValueType::Float(*a - *b)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación - inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::Multiply => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => Ok(ValueType::Int(*a * *b)),
                    (ValueType::Float(a), ValueType::Float(b)) => Ok(ValueType::Float(*a * *b)),
                    (ValueType::Int(a), ValueType::Float(b)) => Ok(ValueType::Float(*a as f64 * *b)),
                    (ValueType::Float(a), ValueType::Int(b)) => Ok(ValueType::Float(*a * *b as f64)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación * inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::Divide => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => {
                        if *b == 0 {
                            return Err(CompileError::semantic_error(1, 1, "División por cero".to_string()));
                        }
                        Ok(ValueType::Float(*a as f64 / *b as f64))
                    }
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        if *b == 0.0 {
                            return Err(CompileError::semantic_error(1, 1, "División por cero".to_string()));
                        }
                        Ok(ValueType::Float(*a / *b))
                    }
                    (ValueType::Int(a), ValueType::Float(b)) => Ok(ValueType::Float(*a as f64 / *b)),
                    (ValueType::Float(a), ValueType::Int(b)) => Ok(ValueType::Float(*a / *b as f64)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación / inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::Modulo => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => {
                        if *b == 0 {
                            return Err(CompileError::semantic_error(1, 1, "Módulo por cero".to_string()));
                        }
                        Ok(ValueType::Int(*a % *b))
                    }
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación % inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::Equal => {
                Ok(ValueType::Bool(self.compare_values(&left, &right)))
            }
            crate::compile_system::BinaryOperator::NotEqual => {
                Ok(ValueType::Bool(!self.compare_values(&left, &right)))
            }
            crate::compile_system::BinaryOperator::LessThan => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => Ok(ValueType::Bool(*a < *b)),
                    (ValueType::Float(a), ValueType::Float(b)) => Ok(ValueType::Bool(*a < *b)),
                    (ValueType::Int(a), ValueType::Float(b)) => Ok(ValueType::Bool((*a as f64) < *b)),
                    (ValueType::Float(a), ValueType::Int(b)) => Ok(ValueType::Bool(*a < *b as f64)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación < inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::LessThanOrEqual => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => Ok(ValueType::Bool(*a <= *b)),
                    (ValueType::Float(a), ValueType::Float(b)) => Ok(ValueType::Bool(*a <= *b)),
                    (ValueType::Int(a), ValueType::Float(b)) => Ok(ValueType::Bool(*a as f64 <= *b)),
                    (ValueType::Float(a), ValueType::Int(b)) => Ok(ValueType::Bool(*a <= *b as f64)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación <= inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::GreaterThan => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => Ok(ValueType::Bool(*a > *b)),
                    (ValueType::Float(a), ValueType::Float(b)) => Ok(ValueType::Bool(*a > *b)),
                    (ValueType::Int(a), ValueType::Float(b)) => Ok(ValueType::Bool(*a as f64 > *b)),
                    (ValueType::Float(a), ValueType::Int(b)) => Ok(ValueType::Bool(*a > *b as f64)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación > inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::GreaterThanOrEqual => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => Ok(ValueType::Bool(*a >= *b)),
                    (ValueType::Float(a), ValueType::Float(b)) => Ok(ValueType::Bool(*a >= *b)),
                    (ValueType::Int(a), ValueType::Float(b)) => Ok(ValueType::Bool(*a as f64 >= *b)),
                    (ValueType::Float(a), ValueType::Int(b)) => Ok(ValueType::Bool(*a >= *b as f64)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación >= inválida".to_string())),
                }
            }
            crate::compile_system::BinaryOperator::And => {
                Ok(ValueType::Bool(self.to_bool(&left) && self.to_bool(&right)))
            }
            crate::compile_system::BinaryOperator::Or => {
                Ok(ValueType::Bool(self.to_bool(&left) || self.to_bool(&right)))
            }
            _ => Err(CompileError::type_mismatch(1, 1, "Operador no implementado".to_string())),
        }
    }

    /// Ejecuta una operación unaria
    fn execute_unary_op(&self, op: &crate::compile_system::UnaryOperator, operand: ValueType) -> Result<ValueType, CompileError> {
        match op {
            crate::compile_system::UnaryOperator::Minus | crate::compile_system::UnaryOperator::Negate => {
                match operand {
                    ValueType::Int(i) => Ok(ValueType::Int(-i)),
                    ValueType::Float(f) => Ok(ValueType::Float(-f)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operador - inválido".to_string())),
                }
            }
            crate::compile_system::UnaryOperator::Not => {
                Ok(ValueType::Bool(!self.to_bool(&operand)))
            }
            crate::compile_system::UnaryOperator::BitwiseNot => {
                match operand {
                    ValueType::Int(i) => Ok(ValueType::Int(!i)),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operador ~ inválido".to_string())),
                }
            }
        }
    }

    /// Ejecuta una operación aritmética
    fn execute_arithmetic_op(&self, left: ValueType, op: &crate::compile_system::ArithmeticOperator, right: ValueType) -> Result<ValueType, CompileError> {
        match op {
            crate::compile_system::ArithmeticOperator::Add => self.execute_binary_op(left, &crate::compile_system::BinaryOperator::Plus, right),
            crate::compile_system::ArithmeticOperator::Subtract => self.execute_binary_op(left, &crate::compile_system::BinaryOperator::Minus, right),
            crate::compile_system::ArithmeticOperator::Multiply => self.execute_binary_op(left, &crate::compile_system::BinaryOperator::Multiply, right),
            crate::compile_system::ArithmeticOperator::Divide => self.execute_binary_op(left, &crate::compile_system::BinaryOperator::Divide, right),
            crate::compile_system::ArithmeticOperator::Modulo => self.execute_binary_op(left, &crate::compile_system::BinaryOperator::Modulo, right),
            crate::compile_system::ArithmeticOperator::Power => {
                match (&left, &right) {
                    (ValueType::Int(a), ValueType::Int(b)) => {
                        if *b < 0 {
                            return Err(CompileError::semantic_error(1, 1, "Potencia con exponente negativo no soportada".to_string()));
                        }
                        Ok(ValueType::Int((*a as i128).pow(*b as u32) as i64))
                    }
                    (ValueType::Float(a), ValueType::Float(b)) => Ok(ValueType::Float((*a).powf(*b))),
                    _ => Err(CompileError::type_mismatch(1, 1, "Operación ^ inválida".to_string())),
                }
            }
        }
    }

    /// Convierte un valor a su tipo correcto
    fn type_value(&self, value: &ValueType, target_type: &DataType) -> ValueType {
        match target_type {
            DataType::Int => match value {
                ValueType::Int(i) => ValueType::Int(*i),
                ValueType::Float(f) => ValueType::Int(*f as i64),
                ValueType::Bool(b) => ValueType::Int(if *b { 1 } else { 0 }),
                _ => ValueType::Int(0),
            },
            DataType::Float => match value {
                ValueType::Int(i) => ValueType::Float(*i as f64),
                ValueType::Float(f) => ValueType::Float(*f),
                ValueType::Bool(b) => ValueType::Float(if *b { 1.0 } else { 0.0 }),
                _ => ValueType::Float(0.0),
            },
            DataType::String => match value {
                ValueType::String(s) => ValueType::String(s.clone()),
                ValueType::Int(i) => ValueType::String(i.to_string()),
                ValueType::Float(f) => ValueType::String(f.to_string()),
                ValueType::Bool(b) => ValueType::String(if *b { "true".to_string() } else { "false".to_string() }),
                _ => ValueType::String(value.to_string()),
            },
            DataType::Bool => match value {
                ValueType::Bool(b) => ValueType::Bool(*b),
                ValueType::Int(i) => ValueType::Bool(*i != 0),
                ValueType::Float(f) => ValueType::Bool(*f != 0.0),
                ValueType::String(s) => ValueType::Bool(!s.is_empty()),
                _ => ValueType::Bool(false),
            },
            DataType::Array(_) => match value {
                ValueType::Array(_) => value.clone(),
                _ => ValueType::Array(vec![]),
            },
            DataType::Object(_) => match value {
                ValueType::Object(_) => value.clone(),
                _ => ValueType::Object(HashMap::new()),
            },
            DataType::Any => value.clone(),
        }
    }

    /// Compara dos valores para igualdad
    fn compare_values(&self, left: &ValueType, right: &ValueType) -> bool {
        match (left, right) {
            (ValueType::Int(a), ValueType::Int(b)) => a == b,
            (ValueType::Float(a), ValueType::Float(b)) => a == b,
            (ValueType::String(a), ValueType::String(b)) => a == b,
            (ValueType::Bool(a), ValueType::Bool(b)) => a == b,
            (ValueType::Array(a), ValueType::Array(b)) => a == b,
            (ValueType::Object(a), ValueType::Object(b)) => a == b,
            _ => false,
        }
    }

    /// Ejecuta un bloque de código
    pub fn execute_block(&mut self, statements: &[ASTNode]) -> Result<(), CompileError> {
        for stmt in statements {
            self.execute_node(stmt)?;
        }
        Ok(())
    }

    /// Ejecuta una función definida
    pub fn execute_function(&mut self, name: &str, params: HashMap<String, ValueType>) -> Result<ValueType, CompileError> {
        if let Some(func) = self.context.get_function(name) {
            let result = func(params);
            Ok(result)
        } else {
            Err(CompileError::semantic_error(1, 1, format!("Función '{}' no encontrada", name)))
        }
    }

    /// Ejecuta todos los scripts en el contexto
    pub fn execute_all(&mut self) -> Result<(), CompileError> {
        Ok(())
    }

    /// Ejecuta un script desde un AST
    pub fn execute_from_ast(&mut self, ast: &ASTNode) -> Result<ValueType, CompileError> {
        let start_time = std::time::Instant::now();
        let result = self.execute_node(ast)?;
        let duration = start_time.elapsed().as_secs_f32();
        
        self.execution_count += 1;
        self.total_execution_time += duration;
        
        Ok(result)
    }

    /// Ejecuta un script desde código fuente
    pub fn execute_from_source(&mut self, source: &str) -> Result<ValueType, CompileError> {
        // Aquí se integraría con el compilador para obtener el AST
        // Por ahora, ejecutamos un ejemplo básico
        let start_time = std::time::Instant::now();
        let result = self.execute_node(&ASTNode::Block {
            statements: vec![
                ASTNode::VariableDeclaration {
                    name: "x".to_string(),
                    value: Box::new(ASTNode::Literal {
                        value: ValueType::Int(10),
                    }),
                    data_type: None,
                },
                ASTNode::VariableDeclaration {
                    name: "y".to_string(),
                    value: Box::new(ASTNode::Literal {
                        value: ValueType::Int(20),
                    }),
                    data_type: None,
                },
                ASTNode::VariableDeclaration {
                    name: "sum".to_string(),
                    value: Box::new(ASTNode::Arithmetic {
                        left: Box::new(ASTNode::Identifier { name: "x".to_string() }),
                        operator: crate::compile_system::ArithmeticOperator::Add,
                        right: Box::new(ASTNode::Identifier { name: "y".to_string() }),
                    }),
                    data_type: None,
                },
                ASTNode::Print {
                    value: Box::new(ASTNode::Identifier { name: "sum".to_string() }),
                },
            ],
        })?;
        
        let duration = start_time.elapsed().as_secs_f32();
        
        self.execution_count += 1;
        self.total_execution_time += duration;
        
        Ok(result)
    }

    /// Obtiene estadísticas de ejecución
    pub fn get_stats(&self) -> (usize, f32) {
        (self.execution_count, self.total_execution_time)
    }

    /// Limpia el contexto de ejecución
    pub fn clear(&mut self) {
        self.context = ExecutionContext::new();
        self.execution_count = 0;
        self.total_execution_time = 0.0;
    }
}

impl Default for ScriptExecutor {
    fn default() -> Self {
        Self::new()
    }
}

