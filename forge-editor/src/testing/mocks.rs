//! Mock and stub utilities for testing

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Mock trait
#[macro_export]
macro_rules! mock {
    ($trait_name:ident, $($method_name:ident $(, $($arg_name:ident : $arg_type:ty)*)* $(= $default:expr)?),*) => {
        impl $crate::testing::mock::MockTrait for $trait_name {
            fn $method_name<$($arg_type),*>(&self, $($arg_name: $arg_type),*) -> Result<
                $( $default )?, 
                $crate::testing::mock::MockError
            > {
                Ok($default)
            }
        }
    };
}

/// Mock trait implementation
pub trait MockTrait {
    fn mock_method(&self) -> Result<(), MockError>;
}

/// Mock error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockError {
    pub message: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
}

impl MockError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            expected: None,
            actual: None,
        }
    }

    pub fn with_expected(mut self, expected: String) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn with_actual(mut self, actual: String) -> Self {
        self.actual = Some(actual);
        self
    }
}

impl std::fmt::Display for MockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockError: {}", self.message)?;
        if let Some(ref expected) = self.expected {
            write!(f, " (expected: {})", expected)?;
        }
        if let Some(ref actual) = self.actual {
            write!(f, " (actual: {})", actual)?;
        }
        Ok(())
    }
}

impl std::error::Error for MockError {}

/// Stub
pub struct Stub<T> {
    pub value: Option<T>,
    pub call_count: usize,
    pub last_call: Option<T>,
}

impl<T: Clone> Stub<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            call_count: 0,
            last_call: None,
        }
    }

    pub fn with_value(value: T) -> Self {
        Self {
            value: Some(value),
            call_count: 0,
            last_call: None,
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    pub fn call(&mut self) -> Option<T> {
        self.call_count += 1;
        self.last_call = self.value.take();
        self.last_call.clone()
    }

    pub fn call_with<F>(&mut self, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        self.call_count += 1;
        let result = f();
        self.last_call = Some(result.clone());
        Some(result)
    }

    pub fn reset(&mut self) {
        self.value = None;
        self.call_count = 0;
        self.last_call = None;
    }

    pub fn call_count(&self) -> usize {
        self.call_count
    }

    pub fn last_call(&self) -> Option<&T> {
        self.last_call.as_ref()
    }

    pub fn is_called(&self) -> bool {
        self.call_count > 0
    }
}

impl<T: Clone> Stub<T> {
    pub fn call_with_clone<F>(&mut self, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        self.call_count += 1;
        let result = f();
        self.last_call = Some(result.clone());
        Some(result)
    }
}

/// StubMap for storing stubs
pub struct StubMap<K, V> {
    pub data: HashMap<K, Stub<V>>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> StubMap<K, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, Stub::with_value(value));
    }

    pub fn get(&self, key: &K) -> Option<&Stub<V>> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut Stub<V>> {
        self.data.get_mut(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<Stub<V>> {
        self.data.remove(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn reset_all(&mut self) {
        for stub in self.data.values_mut() {
            stub.reset();
        }
    }
}

/// MockClock for time-based testing
pub struct MockClock {
    pub current_time: SystemTime,
    pub time_travelled: u64,
}

impl MockClock {
    pub fn new() -> Self {
        Self {
            current_time: SystemTime::now(),
            time_travelled: 0,
        }
    }

    pub fn now(&self) -> SystemTime {
        self.current_time + std::time::Duration::from_millis(self.time_travelled)
    }

    pub fn advance(&mut self, ms: u64) {
        self.time_travelled += ms;
    }

    pub fn reset(&mut self) {
        self.time_travelled = 0;
    }

    pub fn elapsed(&self) -> u64 {
        self.time_travelled
    }
}

/// MockTimer for testing time-dependent code
pub struct MockTimer {
    pub start_time: u64,
    pub paused: bool,
    pub ticks: u64,
}

impl MockTimer {
    pub fn new() -> Self {
        Self {
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            paused: false,
            ticks: 0,
        }
    }

    pub fn start(&mut self) {
        self.start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn elapsed_ms(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now.saturating_sub(self.start_time) * 1000
    }

    pub fn tick(&mut self) {
        if !self.paused {
            self.ticks += 1;
        }
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }

    pub fn reset(&mut self) {
        self.start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.ticks = 0;
    }
}

/// MockLogger for testing logging
pub struct MockLogger {
    pub logs: Vec<LogEntry>,
    pub level: LogLevel,
}

impl MockLogger {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            level: LogLevel::Info,
        }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn info(&mut self, message: &str) {
        if LogLevel::Info <= self.level {
            self.logs.push(LogEntry::new(LogLevel::Info, message));
        }
    }

    pub fn warn(&mut self, message: &str) {
        if LogLevel::Warn <= self.level {
            self.logs.push(LogEntry::new(LogLevel::Warn, message));
        }
    }

    pub fn error(&mut self, message: &str) {
        if LogLevel::Error <= self.level {
            self.logs.push(LogEntry::new(LogLevel::Error, message));
        }
    }

    pub fn debug(&mut self, message: &str) {
        if LogLevel::Debug <= self.level {
            self.logs.push(LogEntry::new(LogLevel::Debug, message));
        }
    }

    pub fn trace(&mut self, message: &str) {
        if LogLevel::Trace <= self.level {
            self.logs.push(LogEntry::new(LogLevel::Trace, message));
        }
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }

    pub fn logs(&self) -> &[LogEntry] {
        &self.logs
    }

    pub fn log_count(&self) -> usize {
        self.logs.len()
    }
}

/// Log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: u64,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: &str) -> Self {
        Self {
            level,
            message: message.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

/// Test fixture builder
pub struct FixtureBuilder<T> {
    pub data: Option<T>,
    pub setup: Option<fn()>,
    pub teardown: Option<fn()>,
    pub config: Option<FixtureConfig>,
}

impl<T> FixtureBuilder<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            setup: None,
            teardown: None,
            config: None,
        }
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_setup(mut self, setup: fn()) -> Self {
        self.setup = Some(setup);
        self
    }

    pub fn with_teardown(mut self, teardown: fn()) -> Self {
        self.teardown = Some(teardown);
        self
    }

    pub fn with_config(mut self, config: FixtureConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Fixture<T> {
        Fixture {
            data: self.data,
            setup: self.setup,
            teardown: self.teardown,
            config: self.config,
        }
    }
}

/// Fixture configuration
#[derive(Debug, Clone)]
pub struct FixtureConfig {
    pub name: String,
    pub auto_cleanup: bool,
    pub allow_parallel: bool,
}

impl Default for FixtureConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            auto_cleanup: true,
            allow_parallel: true,
        }
    }
}

/// Fixture
pub struct Fixture<T> {
    pub data: Option<T>,
    pub setup: Option<fn()>,
    pub teardown: Option<fn()>,
    pub config: Option<FixtureConfig>,
}

impl<T> Fixture<T> {
    pub fn setup(&self) {
        if let Some(setup) = &self.setup {
            setup();
        }
    }

    pub fn teardown(&self) {
        if let Some(teardown) = &self.teardown {
            teardown();
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    pub fn is_configured(&self) -> bool {
        self.config.is_some()
    }
}

