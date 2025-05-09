# ESP32-C6 Rust Firmware

Rust-based firmware for a simple IoT device using the ESP32-C6 microcontroller.  
Designed to measure fluid flow using a GPIO-based sensor and enter light sleep for low power consumption.

## Structure
firmware/  
├── .cargo/  
│   └── config.toml # Custom options for cargo builds (target, runner, etc.)  
├── src/  
│   ├── board/ # Sensor and timing logic  
│   │   ├── mod.rs  
│   │   ├── sensor.rs  
│   │   └── timer.rs  
│   ├── communication/ # Handles Wi-Fi  
│   │   ├── mod.rs  
│   │   ├── network.rs  
│   │   └── wifi.rs  
│   ├── power/ # Sleep and wakeup logic  
│   │   ├── mod.rs  
│   │   └── sleep.rs  
│   ├── util/ # Utilities  
│   │   ├── mod.rs  
│   │   └── message.rs  
│   └── main.rs # Entry point and runtime loop  
├── .gitignore  
├── Cargo.toml # Project manifest  
├── README.md  
├── rust-project.json # Neovim rust-analyzer configuration  
└── rust-toolchain.toml # Specifies the exact Rust toolchain version  

## Highlights

- **Low Power**: Enters light sleep and wakes on GPIO interrupt from a flow sensor.
- **Accurate Measurement**: Timestamped event counting using RTC.
- **Modular Design**: Separate modules for communication, measurement, power, and utilities.
- **Rust Embedded**: Uses `no_std`, `esp-hal`, and `esp-backtrace` for bare-metal reliability.

## Usage
Build project with `SSID="****" PASSWORD="****" DEVICE_ID="****" cargo build --release`, and run it with `espflash flash target/riscv32imc-unknown-none-elf/release/firmware --monitor`.
This way you can build this firmware on server, and flash it locally on end-user machine with `espflash flash firmware --monitor`.
