use eframe::{egui, epi};
use hex;
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use libsm::sm3::hash::Sm3Hash;
use std::fmt::Write;

#[derive(PartialEq)]
enum InputType {
    Text,
    Hex,
}

#[derive(PartialEq)]
enum HashType {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Sm3,
}

struct HashCalculator {
    input: String,
    input_type: InputType,
    hash_type: HashType,
    result: String,
}

impl Default for HashCalculator {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_type: InputType::Text,
            hash_type: HashType::Sha256,
            result: String::new(),
        }
    }
}

impl epi::App for HashCalculator {
    fn name(&self) -> &str {
        "哈希计算器"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("哈希计算器");
            ui.horizontal(|ui| {
                ui.label("输入类型:");
                ui.radio_value(&mut self.input_type, InputType::Text, "文本");
                ui.radio_value(&mut self.input_type, InputType::Hex, "十六进制");
            });

            ui.horizontal(|ui| {
                ui.label("哈希算法:");
                ui.radio_value(&mut self.hash_type, HashType::Sha1, "SHA-1");
                ui.radio_value(&mut self.hash_type, HashType::Sha256, "SHA-256");
                ui.radio_value(&mut self.hash_type, HashType::Sha384, "SHA-384");
                ui.radio_value(&mut self.hash_type, HashType::Sha512, "SHA-512");
            });
            
            ui.horizontal(|ui| {
                ui.label("     ");
                ui.radio_value(&mut self.hash_type, HashType::Sha3_256, "SHA3-256");
                ui.radio_value(&mut self.hash_type, HashType::Sha3_384, "SHA3-384");
                ui.radio_value(&mut self.hash_type, HashType::Sha3_512, "SHA3-512");
                ui.radio_value(&mut self.hash_type, HashType::Sm3, "SM3");
            });

            ui.add_space(10.0);
            
            ui.label("输入:");
            let text_edit = ui.text_edit_multiline(&mut self.input);

            if text_edit.changed() {
                self.calculate_hash();
            }

            if ui.button("计算哈希").clicked() {
                self.calculate_hash();
            }

            ui.add_space(10.0);
            ui.label("结果 (十六进制):");
            ui.text_edit_multiline(&mut self.result);

            if ui.button("复制结果").clicked() {
                ui.output_mut(|o| o.copied_text = self.result.clone());
            }
        });
    }
}

impl HashCalculator {
    fn calculate_hash(&mut self) {
        let input_bytes = match self.input_type {
            InputType::Text => self.input.as_bytes().to_vec(),
            InputType::Hex => match hex::decode(self.input.replace(" ", "")) {
                Ok(bytes) => bytes,
                Err(_) => {
                    self.result = "无效的十六进制输入".to_string();
                    return;
                }
            },
        };

        self.result = match self.hash_type {
            HashType::Sha1 => {
                let mut hasher = Sha1::new();
                hasher.update(&input_bytes);
                let result = hasher.finalize();
                to_hex_string(&result)
            }
            HashType::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(&input_bytes);
                let result = hasher.finalize();
                to_hex_string(&result)
            }
            HashType::Sha384 => {
                let mut hasher = Sha384::new();
                hasher.update(&input_bytes);
                let result = hasher.finalize();
                to_hex_string(&result)
            }
            HashType::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(&input_bytes);
                let result = hasher.finalize();
                to_hex_string(&result)
            }
            HashType::Sha3_256 => {
                let mut hasher = Sha3_256::new();
                hasher.update(&input_bytes);
                let result = hasher.finalize();
                to_hex_string(&result)
            }
            HashType::Sha3_384 => {
                let mut hasher = Sha3_384::new();
                hasher.update(&input_bytes);
                let result = hasher.finalize();
                to_hex_string(&result)
            }
            HashType::Sha3_512 => {
                let mut hasher = Sha3_512::new();
                hasher.update(&input_bytes);
                let result = hasher.finalize();
                to_hex_string(&result)
            }
            HashType::Sm3 => {
                let mut sm3 = Sm3Hash::new(&input_bytes);
                let result = sm3.get_hash();
                let mut hex_string = String::new();
                for byte in result.iter() {
                    write!(hex_string, "{:02x}", byte).unwrap();
                }
                hex_string
            }
        };
    }
}

fn to_hex_string<T: AsRef<[u8]>>(bytes: T) -> String {
    let bytes = bytes.as_ref();
    let mut hex_string = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        Box::new(HashCalculator::default()),
        options,
    )
} 