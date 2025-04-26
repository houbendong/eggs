# Eggs

A utility toolkit for cryptographic operations and security measurements simulation.

## Features

### Hash Calculator

- Support for multiple hash algorithms:
  - SHA-1
  - SHA-256
  - SHA-384
  - SHA-512
  - SHA3-256
  - SHA3-384
  - SHA3-512
  - SM3
- Text and hexadecimal input modes
- Real-time hash calculation
- Copy results with one click

### Boot Replay Metrics Simulator

- Simulate Platform Configuration Register (PCR) measurements
- Support for different hash algorithms (SHA1, SHA256)
- Manual entry of measurements
- File import for batch processing of measurements
- Real-time PCR value updates
- Visual representation of measurement logs

## Usage

### Hash Calculator

1. Select the input type (text or hexadecimal)
2. Choose a hash algorithm
3. Enter the content to hash in the input field
4. Results are displayed automatically
5. Click "Copy Result" to copy the hash value

### Boot Replay Simulator

1. Select the hash algorithm (SHA1 or SHA256)
2. Choose PCR mode (single or all)
3. Enter measurements manually or import from a file
4. View updated PCR values and measurement logs
5. Reset PCRs as needed

## Building and Running

```bash
# Clone the project
git clone https://github.com/yourusername/eggs.git
cd eggs

# Run in debug mode
cargo run

# Build release version
cargo build --release
```

## Dependencies

- eframe - GUI framework
- sha1, sha2, sha3 - SHA family hash algorithms
- libsm - Chinese SM3 hash algorithm
- hex - Hexadecimal conversion
- rfd - File dialog library

## Project Structure

- `src/algorithms/` - Hash algorithm implementations
- `src/models/` - Data structures and models
- `src/ui/` - User interface components
- `src/utils/` - Utility functions and helpers
