use eframe::egui;
use crate::models::pcr_simulator::{PcrSimulator, HashAlgorithm};
use crate::utils::file_processor::MeasurementFileProcessor;
use rfd::FileDialog;

/// Boot Replay Simulator - UI Part
pub struct BootReplayApp {
    // Algorithm selection
    selected_algorithm: HashAlgorithm,
    algorithm_options: Vec<&'static str>,
    
    // PCR options
    selected_pcr: usize,
    
    // UI state control
    show_details: bool,
    input_mode: InputMode,
    
    // Manual input related
    manual_new_measurement: String,
    manual_error: Option<String>,
    
    // File input related
    file_path: Option<String>,
    show_file_dialog: bool,
    selected_measurements: Vec<bool>,
    
    // Simulator instance
    simulator: PcrSimulator,
    file_processor: MeasurementFileProcessor,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InputMode {
    Manual,
    File,
}

impl Default for BootReplayApp {
    fn default() -> Self {
        let default_algorithm = HashAlgorithm::SHA256;
        
        Self {
            selected_algorithm: default_algorithm.clone(),
            algorithm_options: HashAlgorithm::all_algorithms(),
            selected_pcr: 0,
            show_details: false,
            input_mode: InputMode::Manual,
            manual_new_measurement: String::new(),
            manual_error: None,
            file_path: None,
            show_file_dialog: false,
            selected_measurements: Vec::new(),
            simulator: PcrSimulator::new(default_algorithm),
            file_processor: MeasurementFileProcessor::new(),
        }
    }
}

impl BootReplayApp {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top title and description
            ui.vertical_centered(|ui| {
                ui.heading("Boot Measurement Replay Simulator");
                ui.label("Simulate TPM PCR extend operations and verify measurements");
                ui.add_space(10.0);
            });
            
            // Main control area
            ui.group(|ui| {
                // Algorithm selection
                ui.horizontal(|ui| {
                    ui.label("Hash Algorithm:");
                    
                    let current_algorithm = match self.selected_algorithm {
                        HashAlgorithm::SHA1 => "SHA1",
                        HashAlgorithm::SHA256 => "SHA256",
                        HashAlgorithm::SHA384 => "SHA384",
                        HashAlgorithm::SHA512 => "SHA512",
                        HashAlgorithm::SHA3_256 => "SHA3-256",
                        HashAlgorithm::SHA3_384 => "SHA3-384",
                        HashAlgorithm::SHA3_512 => "SHA3-512",
                        HashAlgorithm::SM3 => "SM3",
                    };
                    
                    // Create algorithm dropdown menu
                    egui::ComboBox::from_id_source("algorithm_select")
                        .selected_text(current_algorithm)
                        .show_ui(ui, |ui| {
                            for &alg_name in &self.algorithm_options {
                                let selected = ui.selectable_label(
                                    current_algorithm == alg_name,
                                    alg_name
                                ).clicked();
                                
                                if selected {
                                    if let Some(algorithm) = HashAlgorithm::from_str(alg_name) {
                                        self.selected_algorithm = algorithm.clone();
                                        self.simulator.change_algorithm(algorithm);
                                    }
                                }
                            }
                        });
                });
                
                ui.add_space(5.0);
                
                // PCR selection
                ui.horizontal(|ui| {
                    ui.label("PCR Index:");
                    let pcr_range = 0..24;
                    
                    egui::ComboBox::from_id_source("pcr_select")
                        .selected_text(format!("PCR {}", self.selected_pcr))
                        .show_ui(ui, |ui| {
                            for pcr in pcr_range {
                                if ui.selectable_label(self.selected_pcr == pcr, format!("PCR {}", pcr)).clicked() {
                                    self.selected_pcr = pcr;
                                }
                            }
                        });
                });
                
                ui.add_space(5.0);
                
                // Display current PCR value
                if let Ok(pcr_value) = self.simulator.get_pcr_hex_string(self.selected_pcr) {
                    ui.horizontal(|ui| {
                        ui.label("Current PCR Value:");
                        ui.monospace(pcr_value);
                    });
                }
                
                ui.add_space(10.0);
                
                // Input mode selection
                ui.horizontal(|ui| {
                    ui.label("Input Mode:");
                    
                    if ui.selectable_label(self.input_mode == InputMode::Manual, "Manual Input").clicked() {
                        self.input_mode = InputMode::Manual;
                    }
                    
                    if ui.selectable_label(self.input_mode == InputMode::File, "File Input").clicked() {
                        self.input_mode = InputMode::File;
                    }
                });
                
                ui.add_space(5.0);
                
                // Display different UI based on selected input mode
                match self.input_mode {
                    InputMode::Manual => self.show_manual_mode(ui),
                    InputMode::File => self.show_file_mode(ui, ctx),
                }
            });
            
            // Bottom display measurement log
            if self.show_details {
                self.show_measurement_log(ui);
            }
            
            // Bottom control buttons
            ui.horizontal(|ui| {
                if ui.button("Reset PCR").clicked() {
                    self.simulator.reset();
                }
                
                if ui.button(if self.show_details { "Hide Log" } else { "Show Log" }).clicked() {
                    self.show_details = !self.show_details;
                }
            });
        });
        
        // Handle file dialog
        if self.show_file_dialog {
            self.show_file_dialog = false;
            if let Some(path) = FileDialog::new()
                .add_filter("Measurement Files", &["txt", "log", "bin"])
                .set_title("Select Measurement File")
                .pick_file()
            {
                let path_str = path.to_string_lossy().to_string();
                if let Err(_err) = self.file_processor.load_file(&path_str) {
                    // Report error
                } else {
                    self.file_path = Some(path_str);
                    self.selected_measurements = vec![false; self.file_processor.get_parsed_measurements().len()];
                }
            }
        }
    }

    /// Display manual input mode UI
    fn show_manual_mode(&mut self, ui: &mut egui::Ui) {
        // Measurement value input
        ui.horizontal(|ui| {
            ui.label("Measurement:");
            let text_edit = ui.text_edit_singleline(&mut self.manual_new_measurement);
            
            if text_edit.changed() {
                self.manual_error = None;
            }
            
            if ui.button("Apply").clicked() {
                // Validate input
                let input = self.manual_new_measurement.trim();
                if input.is_empty() {
                    self.manual_error = Some("Please enter a valid measurement".to_string());
                } else {
                    // Try to add measurement
                    match self.simulator.add_measurement(
                        format!("Manual Input: {}", input),
                        input,
                        self.selected_pcr
                    ) {
                        Ok(_) => {
                            self.manual_new_measurement.clear();
                        }
                        Err(err) => {
                            self.manual_error = Some(err);
                        }
                    }
                }
            }
        });
        
        // Error display
        if let Some(ref error) = self.manual_error {
            ui.colored_label(egui::Color32::RED, error);
        }
        
        ui.add_space(5.0);
        
        // Brief description
        ui.label("Enter a measurement in hexadecimal format, then click Apply to extend the PCR.");
    }

    /// Display file input mode UI
    fn show_file_mode(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // File selection
        ui.horizontal(|ui| {
            if ui.button("Select File").clicked() {
                self.show_file_dialog = true;
            }
            
            if let Some(ref path) = self.file_path {
                ui.label(format!("File: {}", path));
            } else {
                ui.label("No file selected");
            }
        });
        
        // Display error
        if let Some(error) = self.file_processor.get_error() {
            ui.colored_label(egui::Color32::RED, error);
        }
        
        ui.add_space(5.0);
        
        // Display file content and parsed measurements
        if let Some(_) = self.file_path {
            let file_content = self.file_processor.get_file_content();
            let parsed_measurements = self.file_processor.get_parsed_measurements();
            
            // If there are parsed measurements
            if !parsed_measurements.is_empty() {
                ui.label(format!("Parsed {} valid measurements from file", parsed_measurements.len()));
                
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    // Update selection status vector size
                    if self.selected_measurements.len() != parsed_measurements.len() {
                        self.selected_measurements = vec![false; parsed_measurements.len()];
                    }
                    
                    // Display select all/none buttons
                    ui.horizontal(|ui| {
                        if ui.button("Select All").clicked() {
                            for selected in &mut self.selected_measurements {
                                *selected = true;
                            }
                        }
                        
                        if ui.button("Deselect All").clicked() {
                            for selected in &mut self.selected_measurements {
                                *selected = false;
                            }
                        }
                    });
                    
                    // Display each measurement, allow selection
                    for (i, measurement) in parsed_measurements.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.selected_measurements[i], "");
                            ui.colored_label(ctx.style().visuals.text_color(), format!("{}: {}", i+1, measurement));
                        });
                    }
                });
                
                // Apply selected measurements button
                let any_selected = self.selected_measurements.iter().any(|&selected| selected);
                if ui.add_enabled(any_selected, egui::Button::new("Apply Selected Measurements")).clicked() {
                    // Collect selected values
                    let mut selected_values = Vec::new();
                    for (i, &selected) in self.selected_measurements.iter().enumerate() {
                        if selected {
                            selected_values.push(parsed_measurements[i].clone());
                        }
                    }
                    
                    // Replay these measurements
                    if let Err(err) = self.simulator.replay(self.selected_pcr, &selected_values) {
                        // Handle error
                        println!("Replay error: {}", err);
                    }
                }
            } else if !file_content.is_empty() {
                ui.label("File does not contain valid measurements");
                // Display file content
                ui.group(|ui| {
                    ui.label("File Content:");
                    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        for line in file_content {
                            ui.colored_label(ctx.style().visuals.text_color(), line);
                        }
                    });
                });
            }
        } else {
            ui.label("Please select a file containing measurements to simulate PCR extension");
        }
    }

    /// Display measurement log
    fn show_measurement_log(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Measurement Log");
            
            if self.simulator.measurement_log.is_empty() {
                ui.label("No measurement records");
                return;
            }
            
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                for (i, (description, value, pcr_index)) in self.simulator.measurement_log.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("#{}: PCR{} ", i+1, pcr_index));
                        ui.label(description);
                        ui.monospace(hex::encode(value));
                    });
                }
            });
        });
    }
}