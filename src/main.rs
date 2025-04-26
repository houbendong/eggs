#![windows_subsystem = "windows"]
mod algorithms;
mod ui;
mod models;
mod utils;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "eggs",
        options,
        Box::new(|_cc| Box::new(ui::MainApp::default())),
    )
}
