//! Test runner and execution engine

use crate::testing::test_framework::{
        TestResult, TestStatus, TestLocation, TestFixture, TestFilter, TestSuite, TestReport,
    };
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::SliceRandom;

/// Test runner
#[derive(Debug, Clone)]
pub struct TestRunner {
    pub suite: TestSuite,
    pub filters: Vec<TestFilter>,
    pub parallel: bool,
    pub timeout_ms: u64,
    pub verbose: bool,
    pub randomize: bool,
}

impl TestRunner {
    pub fn new(name: String, parallel: bool, timeout_ms: u64) -> Self {
        Self {
            suite: TestSuite::new(name, parallel, timeout_ms),
            filters: Vec::new(),
            parallel,
            timeout_ms,
            verbose: false,
            randomize: false,
        }
    }

    pub fn with_filter(mut self, filter: TestFilter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_randomize(mut self, randomize: bool) -> Self {
        self.randomize = randomize;
        self
    }

    pub fn add_test(&mut self, name: String, location: TestLocation) {
        self.suite.tests.push(TestResult::new(name, location));
    }

    pub fn add_fixture(&mut self, fixture: TestFixture) {
        self.suite.fixtures.push(fixture);
    }

    pub fn run(&mut self) -> TestReport {
        let start = SystemTime::now();
        self.run_tests();
        let end = SystemTime::now();

        let elapsed = end
            .duration_since(start)
            .unwrap()
            .as_secs_f64()
            * 1000.0;

        TestReport::new(self.suite.clone()).with_elapsed(elapsed)
    }

    fn run_tests(&mut self) {
        let mut tests = self.suite.tests.clone();

        // Apply filters
        tests.retain(|t| {
            self.filters.iter().all(|f| f.matches(&t.name, &t.location.file))
        });

        // Randomize if enabled
        if self.randomize {
            tests.shuffle(&mut rand::thread_rng());
        }

        // Run tests
        for test in tests {
            self.run_single_test(test);
        }
    }

    fn run_single_test(&mut self, test: TestResult) {
        let mut result = test.clone();

        // Setup fixtures
        for fixture in &self.suite.fixtures {
            fixture.setup();
        }

        // Run test
        let start = SystemTime::now();
        let catch_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // Execute test logic
            true
        }));

        let duration = start
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
            * 1000.0;

        result.duration_ms = duration;
        result.end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        match catch_result {
            Ok(_) => {
                result.status = TestStatus::Passed;
            }
            Err(panic_info) => {
                if let Some(s) = panic_info.downcast_ref::<&str>() {
                    result.panic_message = Some((*s).to_string());
                } else if let Some(s) = panic_info.downcast_ref::<String>() {
                    result.panic_message = Some(s.clone());
                } else {
                    result.panic_message = Some("Unknown panic".to_string());
                }
                result.status = TestStatus::Panicked;
            }
        }

        self.suite.add_test(result);

        // Teardown fixtures
        for fixture in &self.suite.fixtures {
            fixture.teardown();
        }
    }

    pub fn suite(&self) -> &TestSuite {
        &self.suite
    }

    pub fn suite_mut(&mut self) -> &mut TestSuite {
        &mut self.suite
    }
}

/// Test execution context
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub variables: HashMap<String, Value>,
    pub scope_stack: Vec<HashMap<String, Value>>,
    pub current_scope: Option<HashMap<String, Value>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            scope_stack: Vec::new(),
            current_scope: None,
        }
    }

    pub fn enter_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
        self.current_scope = self.scope_stack.last().cloned();
    }

    pub fn exit_scope(&mut self) {
        if let Some(_scope) = self.current_scope.take() {
            self.scope_stack.pop();
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(ref scope) = self.current_scope {
            scope.get(name)
        } else {
            self.variables.get(name)
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        if let Some(ref mut scope) = self.current_scope {
            scope.insert(name, value);
        } else {
            self.variables.insert(name, value);
        }
    }

    pub fn remove(&mut self, name: &str) -> Option<Value> {
        if let Some(ref mut scope) = self.current_scope {
            scope.remove(name)
        } else {
            self.variables.remove(name)
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
}

/// Value type
#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Vec(Vec<Value>),
    Map(HashMap<String, Value>),
    None,
}

impl Value {
    pub fn bool(b: bool) -> Self {
        Value::Bool(b)
    }

    pub fn i64(n: i64) -> Self {
        Value::I64(n)
    }

    pub fn f64(n: f64) -> Self {
        Value::F64(n)
    }

    pub fn string(s: String) -> Self {
        Value::String(s)
    }

    pub fn vec(v: Vec<Value>) -> Self {
        Value::Vec(v)
    }

    pub fn map(m: HashMap<String, Value>) -> Self {
        Value::Map(m)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_i64(&self) -> bool {
        matches!(self, Value::I64(_))
    }

    pub fn is_f64(&self) -> bool {
        matches!(self, Value::F64(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_vec(&self) -> bool {
        matches!(self, Value::Vec(_))
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::I64(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Vec(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Map(m) => Some(m),
            _ => None,
        }
    }
}

/// Test reporter
#[derive(Debug, Clone)]
pub struct TestReporter {
    pub output: Vec<String>,
    pub success: bool,
    pub start_time: u64,
    pub end_time: u64,
}

impl TestReporter {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            success: true,
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            end_time: 0,
        }
    }

    pub fn finish(&mut self) {
        self.end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn record(&mut self, message: String) {
        self.output.push(message);
    }

    pub fn record_passed(&mut self, name: &str, duration_ms: f64) {
        self.record(format!("✓ {} ({:.2}ms)", name, duration_ms));
    }

    pub fn record_failed(&mut self, name: &str, error: &str) {
        self.record(format!("✗ {} - {}", name, error));
        self.success = false;
    }

    pub fn record_panicked(&mut self, name: &str, panic: &str) {
        self.record(format!("✗ {} (panic: {})", name, panic));
        self.success = false;
    }

    pub fn record_skipped(&mut self, name: &str, reason: &str) {
        self.record(format!("⊘ {} - {}", name, reason));
    }

    pub fn record_ignored(&mut self, name: &str) {
        self.record(format!("⊗ {} (ignored)", name));
    }

    pub fn summary(&self) -> String {
        let passed = self.output.iter().filter(|s| s.starts_with("✓")).count();
        let failed = self.output.iter().filter(|s| s.starts_with("✗")).count();
        let skipped = self.output.iter().filter(|s| s.starts_with("⊘")).count();
        let ignored = self.output.iter().filter(|s| s.starts_with("⊗")).count();

        format!(
            "
Test Results
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Passed:  {:4}
  Failed:  {:4}
  Skipped: {:4}
  Ignored: {:4}
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total:   {:4}
  Time:    {:.2}s
",
            passed, failed, skipped, ignored,
            passed + failed + skipped + ignored,
            (self.end_time - self.start_time) as f64 / 1000.0
        )
    }
}

impl Default for TestReporter {
    fn default() -> Self {
        Self::new()
    }
}

