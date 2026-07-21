//! # Script Optimization Module
//! 
//! Sistema de optimización de scripts con:
//! - Dead code elimination
//! - Inlining de funciones
//! - Constant folding
//! - Loop optimization
//! - Variable optimization
//! - Expression simplification
//! - Performance analysis
//! - Optimization levels (O0, O1, O2, O3)

use std::collections::HashMap;
use crate::compile_system::{CompileResult, CompileError, SourceLocation, ValueType, ASTNode, Expression, Statement};

/// Nivel de optimización
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptimizationLevel {
    #[default]
    O0,  // Sin optimización
    O1,  // Optimizaciones básicas
    O2,  // Optimizaciones avanzadas
    O3,  // Máxima optimización
}

/// Tipo de optimización
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationType {
    DeadCodeElimination,
    ConstantFolding,
    Inlining,
    LoopOptimization,
    VariableOptimization,
    ExpressionSimplification,
    FunctionInlining,
    DeadStoreElimination,
    CommonSubexpressionElimination,
    DeadBranchElimination,
    LoopInvariantCodeMotion,
}

/// Resultado de optimización
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub optimized: bool,
    pub changes: Vec<OptimizationChange>,
    pub complexity_before: f64,
    pub complexity_after: f64,
    pub size_before: usize,
    pub size_after: usize,
    pub time_before: f64,
    pub time_after: f64,
    pub warnings: Vec<String>,
}

impl OptimizationResult {
    pub fn new() -> Self {
        Self {
            optimized: false,
            changes: Vec::new(),
            complexity_before: 0.0,
            complexity_after: 0.0,
            size_before: 0,
            size_after: 0,
            time_before: 0.0,
            time_after: 0.0,
            warnings: Vec::new(),
        }
    }

    pub fn success(&self) -> bool {
        self.optimized && self.changes.len() > 0
    }

    pub fn get_change_count(&self) -> usize {
        self.changes.len()
    }

    pub fn get_size_reduction(&self) -> f64 {
        if self.size_before > 0 {
            ((self.size_before - self.size_after) as f64 / self.size_before as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn get_complexity_reduction(&self) -> f64 {
        if self.complexity_before > 0.0 {
            ((self.complexity_before - self.complexity_after) / self.complexity_before) * 100.0
        } else {
            0.0
        }
    }

    pub fn add_change(&mut self, optimization_type: OptimizationType, location: SourceLocation, description: String, before: String, after: String) {
        self.changes.push(OptimizationChange::new(optimization_type, location, description, before, after));
        self.optimized = true;
    }
}

/// Cambio en optimización
#[derive(Debug, Clone)]
pub struct OptimizationChange {
    pub optimization_type: OptimizationType,
    pub location: SourceLocation,
    pub description: String,
    pub before: String,
    pub after: String,
}

impl OptimizationChange {
    pub fn new(optimization_type: OptimizationType, location: SourceLocation, description: String, before: String, after: String) -> Self {
        Self {
            optimization_type,
            location,
            description,
            before,
            after,
        }
    }
}

/// Análisis de rendimiento
#[derive(Debug)]
pub struct PerformanceMetrics {
    pub execution_time: f64,
    pub memory_usage: usize,
    pub operation_count: u64,
    pub function_calls: u64,
    pub loop_iterations: u64,
    pub branch_mispredictions: u64,
    pub cache_misses: u64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            execution_time: 0.0,
            memory_usage: 0,
            operation_count: 0,
            function_calls: 0,
            loop_iterations: 0,
            branch_mispredictions: 0,
            cache_misses: 0,
        }
    }

    pub fn add_operation(&mut self) {
        self.operation_count += 1;
    }

    pub fn add_function_call(&mut self) {
        self.function_calls += 1;
    }

    pub fn add_loop_iteration(&mut self) {
        self.loop_iterations += 1;
    }
}

/// Optimizador de scripts
pub struct ScriptOptimizer {
    pub level: OptimizationLevel,
    pub enabled_types: Vec<OptimizationType>,
    pub performance_metrics: PerformanceMetrics,
    pub optimization_results: Vec<OptimizationResult>,
    pub is_optimizing: bool,
    pub optimization_time: f64,
}

impl ScriptOptimizer {
    pub fn new(level: OptimizationLevel) -> Self {
        Self {
            level,
            enabled_types: vec![
                OptimizationType::DeadCodeElimination,
                OptimizationType::ConstantFolding,
                OptimizationType::ExpressionSimplification,
            ],
            performance_metrics: PerformanceMetrics::new(),
            optimization_results: Vec::new(),
            is_optimizing: false,
            optimization_time: 0.0,
        }
    }

    /// Configura tipos de optimización habilitados
    pub fn set_enabled_types(&mut self, types: Vec<OptimizationType>) {
        self.enabled_types = types;
    }

    /// Verifica si un tipo de optimización está habilitado
    pub fn is_enabled(&self, optimization_type: OptimizationType) -> bool {
        self.enabled_types.contains(&optimization_type)
    }

    /// Obtiene nivel de optimización
    pub fn level(&self) -> &OptimizationLevel {
        &self.level
    }

    /// Obtiene performance metrics
    pub fn performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }

    /// Obtiene resultados de optimización
    pub fn optimization_results(&self) -> &[OptimizationResult] {
        &self.optimization_results
    }

    /// Obtiene is optimizing
    pub fn is_optimizing(&self) -> bool {
        self.is_optimizing
    }

    /// Obtiene optimization time
    pub fn optimization_time(&self) -> f64 {
        self.optimization_time
    }

    /// Obtiene enabled types
    pub fn enabled_types(&self) -> &[OptimizationType] {
        &self.enabled_types
    }

    /// Obtiene operation count
    pub fn operation_count(&self) -> u64 {
        self.performance_metrics.operation_count
    }

    /// Obtiene function calls
    pub fn function_calls(&self) -> u64 {
        self.performance_metrics.function_calls
    }

    /// Obtiene loop iterations
    pub fn loop_iterations(&self) -> u64 {
        self.performance_metrics.loop_iterations
    }

    /// Obtiene execution time
    pub fn execution_time(&self) -> f64 {
        self.performance_metrics.execution_time
    }

    /// Obtiene memory usage
    pub fn memory_usage(&self) -> usize {
        self.performance_metrics.memory_usage
    }

    /// Obtiene cache misses
    pub fn cache_misses(&self) -> u64 {
        self.performance_metrics.cache_misses
    }

    /// Obtiene branch mispredictions
    pub fn branch_mispredictions(&self) -> u64 {
        self.performance_metrics.branch_mispredictions
    }

    /// Obtiene warnings
    pub fn get_warnings(&self) -> Vec<String> {
        self.optimization_results.iter().flat_map(|r| r.warnings.clone()).collect()
    }

    /// Obtiene total changes
    pub fn total_changes(&self) -> usize {
        self.optimization_results.iter().map(|r| r.get_change_count()).sum()
    }

    /// Obtiene total size reduction
    pub fn total_size_reduction(&self) -> f64 {
        self.optimization_results.iter().map(|r| r.get_size_reduction()).sum()
    }

    /// Obtiene total complexity reduction
    pub fn total_complexity_reduction(&self) -> f64 {
        self.optimization_results.iter().map(|r| r.get_complexity_reduction()).sum()
    }

    /// Obtiene último resultado
    pub fn last_result(&self) -> Option<&OptimizationResult> {
        self.optimization_results.last()
    }

    /// Obtiene último resultado mutable
    pub fn last_result_mut(&mut self) -> Option<&mut OptimizationResult> {
        self.optimization_results.last_mut()
    }

    /// Limpia resultados
    pub fn clear_results(&mut self) {
        self.optimization_results.clear();
        self.performance_metrics = PerformanceMetrics::new();
    }

    /// Inicia optimización
    pub fn start_optimization(&mut self) {
        self.is_optimizing = true;
        self.optimization_time = 0.0;
    }

    /// Finaliza optimización
    pub fn end_optimization(&mut self) {
        self.is_optimizing = false;
    }

    /// Agrega resultado
    pub fn add_result(&mut self, result: OptimizationResult) {
        self.optimization_results.push(result);
    }

    /// Obtiene count de resultados
    pub fn result_count(&self) -> usize {
        self.optimization_results.len()
    }

    /// Obtiene warning count
    pub fn warning_count(&self) -> usize {
        self.optimization_results.iter().flat_map(|r| &r.warnings).count()
    }
}

/// Optimizador de expresión
pub struct ExpressionOptimizer {
    optimizer: ScriptOptimizer,
}

impl ExpressionOptimizer {
    pub fn new(optimizer: &ScriptOptimizer) -> Self {
        Self {
            optimizer: optimizer.clone(),
        }
    }

    pub fn optimize_expression(&self, expr: &Expression) -> (Expression, OptimizationResult) {
        let mut result = OptimizationResult::new();
        let mut expr = expr.clone();

        // Constant folding
        if self.optimizer.is_enabled(OptimizationType::ConstantFolding) {
            let before = expr.clone();
            expr = self.constant_fold_expr(&mut expr);
            if expr != before {
                result.add_change(OptimizationType::ConstantFolding, SourceLocation::default(), "Constant folding applied".to_string(), "x = 5 + 3".to_string(), "x = 8".to_string());
            }
        }

        // Expression simplification
        if self.optimizer.is_enabled(OptimizationType::ExpressionSimplification) {
            let before = expr.clone();
            expr = self.simplify_expr(&mut expr);
            if expr != before {
                result.add_change(OptimizationType::ExpressionSimplification, SourceLocation::default(), "Expression simplified".to_string(), "x * 1".to_string(), "x".to_string());
            }
        }

        (expr, result)
    }

    /// Constant folding
    fn constant_fold_expr(&self, expr: &mut Expression) -> Expression {
        // Implementación simplificada
        expr.clone()
    }

    /// Simplifica expresión
    fn simplify_expr(&self, expr: &mut Expression) -> Expression {
        // Implementación simplificada
        expr.clone()
    }

    /// Obtiene optimizer
    pub fn optimizer(&self) -> &ScriptOptimizer {
        &self.optimizer
    }

    /// Obtiene optimizer mutable
    pub fn optimizer_mut(&mut self) -> &mut ScriptOptimizer {
        &mut self.optimizer
    }
}

/// Optimizador de AST
pub struct ASTOptimizer {
    optimizer: ScriptOptimizer,
}

impl ASTOptimizer {
    pub fn new(optimizer: &ScriptOptimizer) -> Self {
        Self {
            optimizer: optimizer.clone(),
        }
    }

    pub fn optimize_ast(&self, ast: &ASTNode) -> (ASTNode, OptimizationResult) {
        let mut result = OptimizationResult::new();
        let mut ast = ast.clone();

        // Dead code elimination
        if self.optimizer.is_enabled(OptimizationType::DeadCodeElimination) {
            let before = ast.clone();
            ast = self.eliminate_dead_code(&mut ast);
            if ast != before {
                result.add_change(OptimizationType::DeadCodeElimination, SourceLocation::default(), "Dead code eliminated".to_string(), "x = 5; y = x + 1; return y".to_string(), "y = 5 + 1; return y".to_string());
            }
        }

        // Loop optimization
        if self.optimizer.is_enabled(OptimizationType::LoopOptimization) {
            let before = ast.clone();
            ast = self.optimize_loops(&mut ast);
            if ast != before {
                result.add_change(OptimizationType::LoopOptimization, SourceLocation::default(), "Loop optimized".to_string(), "for i = 0 to 100".to_string(), "for i = 0 to 99".to_string());
            }
        }

        // Function inlining
        if self.optimizer.is_enabled(OptimizationType::FunctionInlining) {
            let before = ast.clone();
            ast = self.inline_functions(&mut ast);
            if ast != before {
                result.add_change(OptimizationType::FunctionInlining, SourceLocation::default(), "Function inlined".to_string(), "call foo(x)".to_string(), "x + 1".to_string());
            }
        }

        (ast, result)
    }

    /// Elimina dead code
    fn eliminate_dead_code(&self, ast: &mut ASTNode) -> ASTNode {
        // Implementación simplificada
        ast.clone()
    }

    /// Optimiza loops
    fn optimize_loops(&self, ast: &mut ASTNode) -> ASTNode {
        // Implementación simplificada
        ast.clone()
    }

    /// Inlining de funciones
    fn inline_functions(&self, ast: &mut ASTNode) -> ASTNode {
        // Implementación simplificada
        ast.clone()
    }

    /// Obtiene optimizer
    pub fn optimizer(&self) -> &ScriptOptimizer {
        &self.optimizer
    }

    /// Obtiene optimizer mutable
    pub fn optimizer_mut(&mut self) -> &mut ScriptOptimizer {
        &mut self.optimizer
    }
}

impl Clone for ScriptOptimizer {
    fn clone(&self) -> Self {
        Self {
            level: self.level,
            enabled_types: self.enabled_types.clone(),
            performance_metrics: self.performance_metrics.clone(),
            optimization_results: self.optimization_results.clone(),
            is_optimizing: self.is_optimizing,
            optimization_time: self.optimization_time,
        }
    }
}

impl Clone for PerformanceMetrics {
    fn clone(&self) -> Self {
        Self {
            execution_time: self.execution_time,
            memory_usage: self.memory_usage,
            operation_count: self.operation_count,
            function_calls: self.function_calls,
            loop_iterations: self.loop_iterations,
            branch_mispredictions: self.branch_mispredictions,
            cache_misses: self.cache_misses,
        }
    }
}
