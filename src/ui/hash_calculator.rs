use eframe::egui;
use crate::algorithms::{InputType, HashType, calculate_hash};

pub struct HashCalculatorApp {
    input: String,
    input_type: InputType,
    hash_type: HashType,
    result: String,
    error_msg: Option<String>,
    copied: bool,
    active_tab: HashCategory,
}

#[derive(PartialEq, Clone, Copy)]
enum HashCategory {
    SHA1,
    SHA2,
    SHA3,
    SM,
}

impl Default for HashCalculatorApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_type: InputType::Text,
            hash_type: HashType::Sha256,
            result: String::new(),
            error_msg: None,
            copied: false,
            active_tab: HashCategory::SHA2,
        }
    }
}

impl HashCalculatorApp {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Title
            ui.vertical_centered(|ui| {
                ui.heading("Hash Calculator");
                ui.add_space(5.0);
            });
            
            // Main content
            ui.group(|ui| {
                // Input type selector
                ui.horizontal(|ui| {
                    ui.label("Input Type:");
                    ui.radio_value(&mut self.input_type, InputType::Text, "Text");
                    ui.radio_value(&mut self.input_type, InputType::Hex, "Hex");
                });
                
                ui.add_space(10.0);
                
                // Hash algorithm categories
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.active_tab == HashCategory::SHA1, "SHA1").clicked() {
                        self.active_tab = HashCategory::SHA1;
                        if self.hash_type != HashType::Sha1 {
                            self.hash_type = HashType::Sha1;
                            self.compute_hash();
                        }
                    }
                    if ui.selectable_label(self.active_tab == HashCategory::SHA2, "SHA2").clicked() {
                        self.active_tab = HashCategory::SHA2;
                        if self.hash_type != HashType::Sha256 {
                            self.hash_type = HashType::Sha256;
                            self.compute_hash();
                        }
                    }
                    if ui.selectable_label(self.active_tab == HashCategory::SHA3, "SHA3").clicked() {
                        self.active_tab = HashCategory::SHA3;
                        if self.hash_type != HashType::Sha3_256 {
                            self.hash_type = HashType::Sha3_256;
                            self.compute_hash();
                        }
                    }
                    if ui.selectable_label(self.active_tab == HashCategory::SM, "SM").clicked() {
                        self.active_tab = HashCategory::SM;
                        if self.hash_type != HashType::Sm3 {
                            self.hash_type = HashType::Sm3;
                            self.compute_hash();
                        }
                    }
                });
                
                ui.add_space(5.0);
                
                // Hash algorithm selection
                match self.active_tab {
                    HashCategory::SHA1 => {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.hash_type, HashType::Sha1, "SHA-1");
                        });
                    },
                    HashCategory::SHA2 => {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.hash_type, HashType::Sha224, "SHA-224");
                            ui.selectable_value(&mut self.hash_type, HashType::Sha256, "SHA-256");
                            ui.selectable_value(&mut self.hash_type, HashType::Sha384, "SHA-384");
                            ui.selectable_value(&mut self.hash_type, HashType::Sha512, "SHA-512");
                        });
                    },
                    HashCategory::SHA3 => {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.hash_type, HashType::Sha3_224, "SHA3-224");
                            ui.selectable_value(&mut self.hash_type, HashType::Sha3_256, "SHA3-256");
                            ui.selectable_value(&mut self.hash_type, HashType::Sha3_384, "SHA3-384");
                            ui.selectable_value(&mut self.hash_type, HashType::Sha3_512, "SHA3-512");
                        });
                    },
                    HashCategory::SM => {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.hash_type, HashType::Sm3, "SM3");
                        });
                    },
                }
                
                ui.add_space(10.0);
                
                // Input field
                ui.label("Input:");
                let text_edit = ui.add_sized(
                    [ui.available_width(), 100.0],
                    egui::TextEdit::multiline(&mut self.input)
                        .hint_text("Enter text or hex to hash...")
                        .desired_width(f32::INFINITY)
                );
                
                if text_edit.changed() {
                    self.compute_hash();
                    self.copied = false;
                }
                
                ui.add_space(10.0);
                
                // Compute button
                if ui.button("Calculate Hash").clicked() {
                    self.compute_hash();
                    self.copied = false;
                }
                
                ui.add_space(10.0);
                
                // Display error if any
                if let Some(ref error) = self.error_msg {
                    ui.colored_label(egui::Color32::RED, error);
                    ui.add_space(5.0);
                }
                
                // Result section
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Result:");
                        
                        // Hash type label
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let hash_name = match self.hash_type {
                                HashType::Sha1 => "SHA-1",
                                HashType::Sha224 => "SHA-224",
                                HashType::Sha256 => "SHA-256",
                                HashType::Sha384 => "SHA-384",
                                HashType::Sha512 => "SHA-512",
                                HashType::Sha3_224 => "SHA3-224",
                                HashType::Sha3_256 => "SHA3-256",
                                HashType::Sha3_384 => "SHA3-384",
                                HashType::Sha3_512 => "SHA3-512",
                                HashType::Sm3 => "SM3",
                            };
                            ui.label(hash_name);
                        });
                    });
                    
                    // Result display
                    let mut result_copy = self.result.clone();
                    ui.add_sized(
                        [ui.available_width(), 80.0],
                        egui::TextEdit::multiline(&mut result_copy)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .interactive(false)
                    );
                    
                    // Copy button with feedback
                    let copy_label = if self.copied { "✓ Copied!" } else { "Copy Result" };
                    if ui.button(copy_label).clicked() {
                        ui.output_mut(|o| o.copied_text = self.result.clone());
                        self.copied = true;
                    }
                });
                
                // Hash length information
                if !self.result.is_empty() {
                    ui.horizontal(|ui| {
                        ui.label(format!("Hash Length: {} characters ({} bytes)", 
                            self.result.len(), 
                            self.result.len() / 2));
                    });
                }
            });
            
            // Information at the bottom
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label("This tool calculates cryptographic hashes using various algorithms.");
                ui.label("Input can be plain text or hexadecimal values.");
            });
        });
    }

    fn compute_hash(&mut self) {
        // Skip hash calculation if input is empty
        if self.input.trim().is_empty() {
            self.result = String::new();
            self.error_msg = None;
            return;
        }
        
        match calculate_hash(&self.input, self.input_type, self.hash_type) {
            Ok(hash) => {
                self.result = hash;
                self.error_msg = None;
            },
            Err(err) => {
                self.result = String::new();
                self.error_msg = Some(err);
            },
        }
    }
} 