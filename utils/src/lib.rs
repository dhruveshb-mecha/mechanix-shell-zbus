use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub interfaces: Interfaces,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Interfaces {
    pub display: Display,
    pub battery: Battery,
    pub network: Network,
    pub cpu: Cpu,
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Display {
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Battery {
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Network {
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Cpu {
    pub device: String,
}


//write a function to parse the yaml file
pub fn parse_yaml() -> Result<BaseConfig, Box<dyn std::error::Error>> {
    //read the yaml file
    let mut file = File::open("./Confing.yml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let base_config: BaseConfig = serde_yaml::from_str(&contents).expect("Failed to deserialize YAML");

    Ok(base_config)
}
