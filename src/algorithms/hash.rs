use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use libsm::sm3::hash::Sm3Hash;
use std::fmt::Write;
use hex;

#[derive(PartialEq, Clone, Copy)]
pub enum HashType {
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Sm3,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InputType {
    Text,
    Hex,
}

pub fn calculate_hash(input: &str, input_type: InputType, hash_type: HashType) -> Result<String, String> {
    let input_bytes = match input_type {
        InputType::Text => input.as_bytes().to_vec(),
        InputType::Hex => match hex::decode(input.replace(" ", "")) {
            Ok(bytes) => bytes,
            Err(_) => return Err("Invalid Hex Input".to_string()),
        },
    };

    let result = match hash_type {
        HashType::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(&input_bytes);
            let result = hasher.finalize();
            to_hex_string(&result)
        }
        HashType::Sha224 => {
            let mut hasher = Sha224::new();
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
        HashType::Sha3_224 => {
            let mut hasher = Sha3_224::new();
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

    Ok(result)
}

fn to_hex_string<T: AsRef<[u8]>>(bytes: T) -> String {
    let bytes = bytes.as_ref();
    let mut hex_string = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
} 