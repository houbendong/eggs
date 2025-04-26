use eframe::egui;
use crate::ui::hash_calculator::HashCalculatorApp;
use crate::ui::boot_replay::BootReplayApp;

/// Application tabs
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tab {
    HashCalculator,
    BootReplay,
}

/// Main application
pub struct MainApp {
    current_tab: Tab,
    hash_calculator: HashCalculatorApp,
    boot_replay: BootReplayApp,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            current_tab: Tab::HashCalculator,
            hash_calculator: HashCalculatorApp::default(),
            boot_replay: BootReplayApp::default(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Draw the top tab bar
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::HashCalculator, "Hash Calculator");
                ui.selectable_value(&mut self.current_tab, Tab::BootReplay, "Boot Measurement Replay");
            });
        });

        // Display the content of the currently selected tab
        match self.current_tab {
            Tab::HashCalculator => self.hash_calculator.update(ctx, frame),
            Tab::BootReplay => self.boot_replay.update(ctx, frame),
        }
    }
} 