use sha2::{Sha256, Sha384, Sha512, Digest};
use sha1::Sha1;
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use libsm::sm3::hash::Sm3Hash;
use hex;

#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    SM3,
}

impl HashAlgorithm {
    pub fn from_str(algorithm: &str) -> Option<Self> {
        match algorithm.to_lowercase().as_str() {
            "sha1" => Some(HashAlgorithm::SHA1),
            "sha256" => Some(HashAlgorithm::SHA256),
            "sha384" => Some(HashAlgorithm::SHA384),
            "sha512" => Some(HashAlgorithm::SHA512),
            "sha3-256" | "sha3_256" => Some(HashAlgorithm::SHA3_256),
            "sha3-384" | "sha3_384" => Some(HashAlgorithm::SHA3_384),
            "sha3-512" | "sha3_512" => Some(HashAlgorithm::SHA3_512),
            "sm3" => Some(HashAlgorithm::SM3),
            _ => None,
        }
    }
    
    pub fn all_algorithms() -> Vec<&'static str> {
        vec![
            "SHA1", 
            "SHA256", 
            "SHA384", 
            "SHA512", 
            "SHA3-256", 
            "SHA3-384", 
            "SHA3-512",
            "SM3"
        ]
    }
    
    pub fn output_size_bytes(&self) -> usize {
        match self {
            HashAlgorithm::SHA1 => 20,
            HashAlgorithm::SHA256 | HashAlgorithm::SHA3_256 | HashAlgorithm::SM3 => 32,
            HashAlgorithm::SHA384 | HashAlgorithm::SHA3_384 => 48,
            HashAlgorithm::SHA512 | HashAlgorithm::SHA3_512 => 64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PcrSimulator {
    pub algorithm: HashAlgorithm,
    pub pcr_values: Vec<Vec<u8>>,
    pub measurement_log: Vec<(String, Vec<u8>, usize)>, // (description, measurement, PCR index)
}

impl PcrSimulator {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        let output_size = algorithm.output_size_bytes();
        
        // Initialize PCR values
        let mut pcr_values = Vec::new();
        for i in 0..24 {
            if i >= 17 && i <= 22 {
                // PCR 17-22 initial value F
                pcr_values.push(vec![0xFF; output_size]);
            } else {
                // PCR 0-16 and 23 initial value 0
                pcr_values.push(vec![0u8; output_size]);
            }
        }

        Self {
            algorithm,
            pcr_values,
            measurement_log: Vec::new(),
        }
    }
    
    /// Calculate hash value (according to selected algorithm)
    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        match self.algorithm {
            HashAlgorithm::SHA1 => {
                let mut hasher = Sha1::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            HashAlgorithm::SHA384 => {
                let mut hasher = Sha384::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            HashAlgorithm::SHA512 => {
                let mut hasher = Sha512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            HashAlgorithm::SHA3_256 => {
                let mut hasher = Sha3_256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            HashAlgorithm::SHA3_384 => {
                let mut hasher = Sha3_384::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            HashAlgorithm::SHA3_512 => {
                let mut hasher = Sha3_512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            HashAlgorithm::SM3 => {
                let mut sm3 = Sm3Hash::new(data);
                sm3.get_hash().to_vec()
            },
        }
    }

    /// Extend PCR
    pub fn extend_pcr(&mut self, pcr_index: usize, measurement: &[u8]) -> Result<(), String> {
        if pcr_index >= self.pcr_values.len() {
            return Err(format!("Invalid PCR index: {}", pcr_index));
        }

        let mut data = Vec::new();
        data.extend_from_slice(&self.pcr_values[pcr_index]);
        data.extend_from_slice(measurement);
        
        // Calculate new PCR value (hash extension)
        let result = self.hash(&data);
        self.pcr_values[pcr_index] = result;
        
        Ok(())
    }

    /// Add new measurement
    pub fn add_measurement(&mut self, description: String, hex_value: &str, pcr_index: usize) -> Result<(), String> {
        let value = hex::decode(hex_value)
            .map_err(|e| format!("Failed to decode measurement: {}", e))?;
            
        self.measurement_log.push((description, value.clone(), pcr_index));
        self.extend_pcr(pcr_index, &value)?;
        
        Ok(())
    }

    /// Reset PCR values
    pub fn reset(&mut self) {
        let output_size = self.algorithm.output_size_bytes();
        
        // Reset PCR values to their initial values
        for i in 0..self.pcr_values.len() {
            if i >= 17 && i <= 22 {
                // PCR 17-22 reset F
                self.pcr_values[i] = vec![0xFF; output_size];
            } else {
                // PCR 0-16 and 23 reset 0
                self.pcr_values[i] = vec![0u8; output_size];
            }
        }
        
        self.measurement_log.clear();
    }

    /// Get PCR hex representation
    pub fn get_pcr_hex_string(&self, index: usize) -> Result<String, String> {
        if index >= self.pcr_values.len() {
            return Err("Invalid PCR index".to_string());
        }
        Ok(hex::encode(&self.pcr_values[index]))
    }
    
    /// Parse hex string
    pub fn parse_hex(hex_str: &str) -> Result<Vec<u8>, String> {
        hex::decode(hex_str.trim().replace(" ", ""))
            .map_err(|e| format!("Invalid hex string: {}", e))
    }

    /// Replay measurement sequence
    pub fn replay(&mut self, pcr_index: usize, measurements: &[String]) -> Result<String, String> {
        // Ensure PCR index is valid
        if pcr_index >= self.pcr_values.len() {
            return Err(format!("Invalid PCR index: {}", pcr_index));
        }

        // Reset target PCR to its initial value
        let output_size = self.algorithm.output_size_bytes();
        if pcr_index >= 17 && pcr_index <= 22 {
            self.pcr_values[pcr_index] = vec![0xFF; output_size];
        } else {
            self.pcr_values[pcr_index] = vec![0u8; output_size];
        }
        
        // Apply all measurements
        for measurement in measurements {
            let value = Self::parse_hex(measurement)?;
            self.extend_pcr(pcr_index, &value)?;
        }

        // Return final PCR value
        self.get_pcr_hex_string(pcr_index)
    }
    
    /// Get all PCR values
    #[allow(dead_code)]
    pub fn get_all_pcr_values(&self) -> Vec<(usize, String)> {
        let mut result = Vec::new();
        for (index, value) in self.pcr_values.iter().enumerate() {
            result.push((index, hex::encode(value)));
        }
        result
    }
    
    /// Change hash algorithm
    pub fn change_algorithm(&mut self, algorithm: HashAlgorithm) {
        self.algorithm = algorithm;
        self.reset(); // Reset all values when changing algorithm
    }
} 