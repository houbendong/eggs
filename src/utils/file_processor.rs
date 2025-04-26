use std::path::PathBuf;
use std::fs;

/// Measurement File Processor - For reading and parsing measurement files
pub struct MeasurementFileProcessor {
    file_path: Option<PathBuf>,
    file_content: Vec<String>,
    parsed_measurements: Vec<String>,
    error: Option<String>,
}

impl MeasurementFileProcessor {
    pub fn new() -> Self {
        Self {
            file_path: None,
            file_content: Vec::new(),
            parsed_measurements: Vec::new(),
            error: None,
        }
    }
    
    /// Load file
    pub fn load_file(&mut self, path_str: &str) -> Result<(), String> {
        self.file_content.clear();
        self.parsed_measurements.clear();
        self.error = None;
        
        match fs::read_to_string(path_str) {
            Ok(content) => {
                // Store file content
                self.file_path = Some(PathBuf::from(path_str));
                self.file_content = content.lines().map(String::from).collect();
                
                // Parse measurements
                self.parse_measurements();
                Ok(())
            },
            Err(err) => {
                let error_msg = format!("Error reading file: {}", err);
                self.error = Some(error_msg.clone());
                Err(error_msg)
            }
        }
    }
    
    /// Parse measurements from file
    fn parse_measurements(&mut self) {
        self.parsed_measurements.clear();
        
        for line in &self.file_content {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
            
            // Check if it's a valid hex string
            if line.chars().all(|c| c.is_digit(16) || c.is_whitespace()) {
                // Remove spaces
                let hex_only = line.replace(" ", "");
                if !hex_only.is_empty() {
                    if let Ok(validated) = self.validate_hex(&hex_only) {
                        self.parsed_measurements.push(validated);
                    }
                }
            }
        }
    }
    
    /// Validate hex measurement
    pub fn validate_hex(&self, hex: &str) -> Result<String, &'static str> {
        let hex = hex.replace(" ", "");
        if hex.len() > 64 {
            return Err("Measurement must be at most 64 hex characters");
        }
        
        // Validate hex characters
        if !hex.chars().all(|c| c.is_digit(16)) {
            return Err("Invalid hex characters");
        }
        
        // Pad to 64 characters
        let padded = format!("{:0>64}", hex);
        Ok(padded)
    }
    
    /// Get file path
    #[allow(dead_code)]
    pub fn get_file_path(&self) -> Option<String> {
        self.file_path.as_ref().map(|p| p.to_string_lossy().to_string())
    }
    
    /// Get file content
    pub fn get_file_content(&self) -> &[String] {
        &self.file_content
    }
    
    /// Get parsed measurements
    pub fn get_parsed_measurements(&self) -> &[String] {
        &self.parsed_measurements
    }
    
    /// Get error message
    pub fn get_error(&self) -> Option<&str> {
        self.error.as_deref()
    }
} 