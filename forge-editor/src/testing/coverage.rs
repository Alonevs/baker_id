use std::collections::HashMap;

/// Coverage Line
#[derive(Debug, Clone, Default)]
pub struct CoverageLine {
    pub line: usize,
    pub executed: bool,
}

impl CoverageLine {
    pub fn new(line: usize) -> Self {
        Self { line, executed: false }
    }

    pub fn execute(&mut self) {
        self.executed = true;
    }
}

/// Coverage Function
#[derive(Debug, Clone, Default)]
pub struct CoverageFunction {
    pub name: String,
    pub covered: bool,
}

impl CoverageFunction {
    pub fn new(name: String) -> Self {
        Self { name, covered: false }
    }

    pub fn execute(&mut self) {
        self.covered = true;
    }

    pub fn add_line(&mut self, line: usize) {
        if !self.covered {
            self.covered = true;
        }
    }
}

/// Coverage Branch
#[derive(Debug, Clone, Default)]
pub struct CoverageBranch {
    pub id: usize,
    pub taken: bool,
}

impl CoverageBranch {
    pub fn new(id: usize) -> Self {
        Self { id, taken: false }
    }

    pub fn take(&mut self) {
        self.taken = true;
    }
}

/// Coverage Report
#[derive(Debug, Clone, Default)]
pub struct CoverageReport {
    pub file: String,
    pub lines: Vec<CoverageLine>,
    pub functions: Vec<CoverageFunction>,
    pub branches: Vec<CoverageBranch>,
    pub total_lines: usize,
    pub covered_lines: usize,
    pub total_functions: usize,
    pub covered_functions: usize,
    pub total_branches: usize,
    pub covered_branches: usize,
    pub line_coverage: f64,
    pub function_coverage: f64,
    pub branch_coverage: f64,
}

impl CoverageReport {
    pub fn new(file: String) -> Self {
        Self {
            file,
            lines: Vec::new(),
            functions: Vec::new(),
            branches: Vec::new(),
            total_lines: 0,
            covered_lines: 0,
            total_functions: 0,
            covered_functions: 0,
            total_branches: 0,
            covered_branches: 0,
            line_coverage: 0.0,
            function_coverage: 0.0,
            branch_coverage: 0.0,
        }
    }

    pub fn add_line(&mut self, line: usize) {
        if self.lines.iter().any(|l| l.line == line) {
            return;
        }
        self.lines.push(CoverageLine::new(line));
    }

    pub fn add_function(&mut self, name: String) {
        if self.functions.iter().any(|f| f.name == name) {
            return;
        }
        self.functions.push(CoverageFunction::new(name));
    }

    pub fn add_branch(&mut self, id: usize) {
        if self.branches.iter().any(|b| b.id == id) {
            return;
        }
        self.branches.push(CoverageBranch::new(id));
    }

    pub fn mark_line_executed(&mut self, line: usize) {
        if let Some(line) = self.lines.iter_mut().find(|l| l.line == line) {
            line.execute();
            self.total_lines += 1;
            if line.executed {
                self.covered_lines += 1;
            }
        }
    }

    pub fn mark_function_executed(&mut self, name: &str) {
        if let Some(func) = self.functions.iter_mut().find(|f| f.name == name) {
            func.execute();
            self.total_functions += 1;
            if func.covered {
                self.covered_functions += 1;
            }
        }
    }

    pub fn mark_branch_taken(&mut self, id: usize) {
        if let Some(branch) = self.branches.iter_mut().find(|b| b.id == id) {
            branch.take();
            self.total_branches += 1;
            if branch.taken {
                self.covered_branches += 1;
            }
        }
    }

    pub fn calculate_coverage(&mut self) {
        if self.total_lines > 0 {
            self.line_coverage = self.covered_lines as f64 / self.total_lines as f64;
        }
        if self.total_functions > 0 {
            self.function_coverage = self.covered_functions as f64 / self.total_functions as f64;
        }
        if self.total_branches > 0 {
            self.branch_coverage = self.covered_branches as f64 / self.total_branches as f64;
        }
    }

    pub fn line_coverage(&self) -> f64 {
        self.line_coverage
    }

    pub fn function_coverage(&self) -> f64 {
        self.function_coverage
    }

    pub fn branch_coverage(&self) -> f64 {
        self.branch_coverage
    }

    pub fn total_coverage(&self) -> f64 {
        (self.line_coverage + self.function_coverage + self.branch_coverage) / 3.0
    }

    pub fn summary(&self) -> String {
        format!(
            "Coverage for '{}': {}% lines, {}% functions, {}% branches",
            self.file,
            self.line_coverage * 100.0,
            self.function_coverage * 100.0,
            self.branch_coverage * 100.0
        )
    }
}

/// Coverage Analyzer
#[derive(Debug, Clone, Default)]
pub struct CoverageAnalyzer {
    pub lines: Vec<usize>,
    pub functions: Vec<String>,
    pub branches: Vec<usize>,
}

impl CoverageAnalyzer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            functions: Vec::new(),
            branches: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: usize) {
        if !self.lines.contains(&line) {
            self.lines.push(line);
        }
    }

    pub fn add_function(&mut self, name: String) {
        if !self.functions.contains(&name) {
            self.functions.push(name);
        }
    }

    pub fn add_branch(&mut self, id: usize) {
        if !self.branches.contains(&id) {
            self.branches.push(id);
        }
    }

    pub fn count_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn count_functions(&self) -> usize {
        self.functions.len()
    }

    pub fn count_branches(&self) -> usize {
        self.branches.len()
    }

    pub fn total_entities(&self) -> usize {
        self.count_lines() + self.count_functions() + self.count_branches()
    }
}

/// Coverage Collector
#[derive(Debug, Clone, Default)]
pub struct CoverageCollector {
    pub reports: HashMap<String, CoverageReport>,
    pub current_file: Option<String>,
}

impl CoverageCollector {
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
            current_file: None,
        }
    }

    pub fn add_report(&mut self, file: String) {
        self.reports.insert(file.clone(), CoverageReport::new(file));
    }

    pub fn get_report(&self, file: &str) -> Option<&CoverageReport> {
        self.reports.get(file)
    }

    pub fn get_report_mut(&mut self, file: &str) -> Option<&mut CoverageReport> {
        self.reports.get_mut(file)
    }

    pub fn mark_line(&mut self, file: &str, line: usize) {
        if let Some(report) = self.get_report_mut(file) {
            report.add_line(line);
        }
    }

    pub fn mark_function(&mut self, file: &str, name: &str) {
        if let Some(report) = self.get_report_mut(file) {
            report.add_function(name.to_string());
        }
    }

    pub fn mark_branch(&mut self, file: &str, id: usize) {
        if let Some(report) = self.get_report_mut(file) {
            report.add_branch(id);
        }
    }

    pub fn calculate_all(&mut self) {
        for report in self.reports.values_mut() {
            report.calculate_coverage();
        }
    }

    pub fn overall_line_coverage(&self) -> f64 {
        if self.reports.is_empty() {
            return 0.0;
        }
        let total = self.reports.values().map(|r| r.total_lines as i64).sum::<i64>();
        let covered = self.reports.values().map(|r| r.covered_lines as i64).sum::<i64>();
        if total == 0 {
            0.0
        } else {
            covered as f64 / total as f64
        }
    }

    pub fn overall_function_coverage(&self) -> f64 {
        if self.reports.is_empty() {
            return 0.0;
        }
        let total = self.reports.values().map(|r| r.total_functions as i64).sum::<i64>();
        let covered = self.reports.values().map(|r| r.covered_functions as i64).sum::<i64>();
        if total == 0 {
            0.0
        } else {
            covered as f64 / total as f64
        }
    }

    pub fn overall_branch_coverage(&self) -> f64 {
        if self.reports.is_empty() {
            return 0.0;
        }
        let total = self.reports.values().map(|r| r.total_branches as i64).sum::<i64>();
        let covered = self.reports.values().map(|r| r.covered_branches as i64).sum::<i64>();
        if total == 0 {
            0.0
        } else {
            covered as f64 / total as f64
        }
    }

    pub fn overall_coverage(&self) -> f64 {
        (self.overall_line_coverage() + self.overall_function_coverage() + self.overall_branch_coverage()) / 3.0
    }

    pub fn summary(&self) -> String {
        let total = self.reports.len();
        let line_cov = self.overall_line_coverage() * 100.0;
        let func_cov = self.overall_function_coverage() * 100.0;
        let branch_cov = self.overall_branch_coverage() * 100.0;
        format!(
            "Coverage across {} files: {}% lines, {}% functions, {}% branches",
            total, line_cov, func_cov, branch_cov
        )
    }
}

