//! Test panel UI

use crate::testing::{
    test_framework::{TestResult, TestStatus, TestLocation},
    test_runner::TestReporter,
    coverage::CoverageReport,
};
use egui::{Context, RichText, Sense, Ui, Vec2};

/// Test panel
#[derive(Default)]
pub struct TestPanel {
    pub show_tests: bool,
    pub show_coverage: bool,
    pub show_filters: bool,
    pub filter_text: String,
    pub selected_tab: TestTab,
    pub tests: Vec<TestResult>,
    pub coverage: Option<CoverageReport>,
    pub reporter: TestReporter,
}

impl TestPanel {
    pub fn new() -> Self {
        Self {
            show_tests: true,
            show_coverage: false,
            show_filters: false,
            filter_text: String::new(),
            selected_tab: TestTab::Tests,
            tests: Vec::new(),
            coverage: None,
            reporter: TestReporter::new(),
        }
    }

    pub fn ui(&mut self, ctx: &Context) {
        egui::SidePanel::left("test_panel")
            .resizable(true)
            .min_width(400.0)
            .max_width(800.0)
            .show(ctx, |ui| {
                ui.heading("Test Panel");

                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(240, 240, 240))
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        self.render_tab_bar(ui);
                        ui.add_space(10.0);

                        match self.selected_tab {
                            TestTab::Tests => self.render_tests_tab(ui),
                            TestTab::Coverage => self.render_coverage_tab(ui),
                            TestTab::Filters => self.render_filters_tab(ui),
                        }
                    });
            });
    }

    fn render_tab_bar(&mut self, ui: &mut Ui) {
        let tab_height = 30.0;

        ui.horizontal(|ui| {
            self.render_tab_button(ui, "Tests", TestTab::Tests, tab_height);
            self.render_tab_button(ui, "Coverage", TestTab::Coverage, tab_height);
            self.render_tab_button(ui, "Filters", TestTab::Filters, tab_height);
        });
    }

    fn render_tab_button(&mut self, ui: &mut Ui, label: &str, tab: TestTab, height: f32) {
        let response = ui.horizontal(|ui| {
            ui.add(egui::Button::new(label).min_size(Vec2::new(80.0, height)))
        }).inner;

        if response.hovered() {
            ui.painter().rect(
                response.rect,
                0.0,
                egui::Color32::from_rgb(200, 200, 200),
                egui::Stroke::NONE,
                egui::StrokeKind::Outside,
            );
        }

        if response.clicked() {
            self.selected_tab = tab;
        }
    }

    fn render_tests_tab(&mut self, ui: &mut Ui) {
        ui.heading("Test Results");

        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(format!(
                "Total: {} | Passed: {} | Failed: {} | Skipped: {}",
                self.tests.len(),
                self.tests.iter().filter(|t| t.is_passed()).count(),
                self.tests.iter().filter(|t| t.is_failed()).count(),
                self.tests.iter().filter(|t| t.is_skipped()).count()
            ));
        });

        ui.add_space(10.0);

        let tests = self.tests.clone();
        egui::ScrollArea::vertical().show(ui, |ui| {
            for test in &tests {
                self.render_test_result(ui, test);
                ui.add_space(5.0);
            }
        });
    }

    fn render_test_result(&mut self, ui: &mut Ui, test: &TestResult) {
        let status_color = match test.status {
            TestStatus::Passed => egui::Color32::GREEN,
            TestStatus::Failed => egui::Color32::RED,
            TestStatus::Panicked => egui::Color32::YELLOW,
            TestStatus::Skipped => egui::Color32::GRAY,
            TestStatus::Ignored => egui::Color32::DARK_GRAY,
        };

        let status_icon = match test.status {
            TestStatus::Passed => "✓",
            TestStatus::Failed => "✗",
            TestStatus::Panicked => "✘",
            TestStatus::Skipped => "⊘",
            TestStatus::Ignored => "⊗",
        };

        ui.horizontal(|ui| {
            ui.label(RichText::new(status_icon).color(status_color));
            ui.label(&test.name);
            ui.label(format!("({:.2}ms)", test.duration_ms));
        });

        if test.is_failed() || test.is_panicked() {
            ui.label(format!(
                "Error: {} {}",
                test.error_message.as_deref().unwrap_or("Unknown error"),
                test.panic_message.as_deref().unwrap_or("")
            ));
            ui.add_space(5.0);
            ui.label(format!(
                "Location: {}:{}:{}",
                test.location.file, test.location.line, test.location.column
            ));
        }
    }

    fn render_coverage_tab(&mut self, ui: &mut Ui) {
        if let Some(ref coverage) = self.coverage {
            ui.heading("Coverage Report");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Line Coverage:").size(14.0));
                ui.label(format!("{:.1}%", coverage.line_coverage()));
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Function Coverage:").size(14.0));
                ui.label(format!("{:.1}%", coverage.function_coverage()));
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Branch Coverage:").size(14.0));
                ui.label(format!("{:.1}%", coverage.branch_coverage()));
            });

            ui.add_space(20.0);

            ui.label(coverage.summary());
        } else {
            ui.label("No coverage data available");
        }
    }

    fn render_filters_tab(&mut self, ui: &mut Ui) {
        ui.heading("Test Filters");

        ui.add_space(10.0);

        ui.label("Filter by test name:");
        ui.add(egui::TextEdit::singleline(&mut self.filter_text)
            .hint_text("Enter test name..."));

        ui.add_space(10.0);

        ui.label("Available tests:");
        ui.add_space(5.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for test in &self.tests {
                ui.selectable_label(
                    test.name.contains(&self.filter_text),
                    &test.name,
                );
            }
        });
    }
}

/// Test tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TestTab {
    #[default]
    Tests,
    Coverage,
    Filters,
}

