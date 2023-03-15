use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    ComPort: Vec<String>,
    ModbusAddress: String,
    Regs: Vec<u16>,
    log_interval: u32,
}

fn main() {
    // Open the configuration file
    let mut file = File::open("config.yml").expect("Failed to open config file");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config file");

    // Deserialize the YAML into a Config struct
    let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    // Extract the Regs field and print it
    let regs = config.Regs;
    println!("{:?}", regs);
    println!("ComPorts: {:?}", config.ComPort);
}
