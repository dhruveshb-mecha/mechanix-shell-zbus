use mecha_battery_ctl::{Battery, PowerSupplyInfo};
use rand::Rng;
use std::process::Command;
use utils::parse_yaml;
use zbus::{
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
};

use crate::handle_power_supply_error;

pub struct PowerBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `BatteryStatusResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct BatteryStatusResponse {
    pub name: String,
    pub r#type: String,
    pub status: String,
    pub present: bool,
    pub voltage_now: u32,
    pub current_now: i32,
    pub capacity: u8,
    pub capacity_level: String,
    pub temp: i32,
    pub technology: String,
    pub charge_full: u32,
    pub charge_now: u32,
    pub charge_full_design: u32,
    pub manufacturer: String,
}

#[cfg(not(feature = "mock"))]
#[interface(name = "Mechanix.Services.Power")]
impl PowerBusInterface {
    pub async fn get_battery_info(&self) -> Result<BatteryStatusResponse, zbus::fdo::Error> {
        //get battery path
        let battery_path = parse_yaml().unwrap().interfaces.battery.device;

        //get battery instance
        let battery = Battery {
            path: format!("{}/uevent", battery_path),
            current_now: format!("{}/current_now", battery_path),
        };

        //get battery status if  there is an error return moke BatteryInfo
        let result = match battery.info() {
            Ok(status) => status,
            Err(e) => {
                return Err(handle_power_supply_error(e));
            }
        };

        Ok((BatteryStatusResponse {
            name: result.name,
            r#type: result.r#type,
            status: result.status,
            present: result.present,
            voltage_now: result.voltage_now,
            current_now: result.current_now,
            capacity: result.capacity,
            capacity_level: result.capacity_level,
            temp: result.temp,
            technology: result.technology,
            charge_full: result.charge_full,
            charge_now: result.charge_now,
            charge_full_design: result.charge_full_design,
            manufacturer: result.manufacturer,
        }))
    }

    //get battery percentage
    pub async fn get_battery_status(&self) -> Result<u8, zbus::fdo::Error> {
        //get battery path
        let battery_path = parse_yaml().unwrap().interfaces.battery.device;

        //get battery instance
        let battery = Battery {
            path: format!("{}/uevent", battery_path),
            current_now: format!("{}/current_now", battery_path),
        };

        //get battery percentage if  there is an error return  by default the sdk returns () we need to return a u8
        let result: u8 = match battery.info() {
            Ok(battery_info) => battery_info.capacity,
            Err(e) => {
                return Err(handle_power_supply_error(e));
            }
        };

        Ok(result)
    }

    //get power usage
    pub async fn get_power_usage(&self) -> Result<i32, zbus::fdo::Error> {
        //get battery path
        let battery_path = parse_yaml().unwrap().interfaces.battery.device;

        //get battery instance
        let battery = Battery {
            path: format!("{}/uevent", battery_path),
            current_now: format!("{}/current_now", battery_path),
        };

        //get power usage if  there is an error return  by default the sdk returns () we need to return a u8
        let result: i32 = match battery.info() {
            Ok(battery_info) => battery_info.current_now,
            Err(e) => {
                return Err(handle_power_supply_error(e));
            }
        };

        Ok(result)
    }

    // get cpu governor
    pub async fn get_cpu_governor(&self) -> Result<String, zbus::fdo::Error> {
        //get cpu path
        let cpu_path = parse_yaml().unwrap().interfaces.cpu.device;

        //command to get governor
        let output = Command::new("cat")
            .arg(format!("{}/cpufreq/scaling_governor", cpu_path))
            .output()
            .expect("Failed to execute cat command");

        //on success return governor
        let _ = match output.status.success() {
            true => {
                let governor = String::from_utf8(output.stdout).unwrap();
                return Ok(governor);
            }
            false => {
                return Err(zbus::fdo::Error::Failed(
                    "Failed to get governor".to_string(),
                ));
            }
        };
    }

    // set cpu governor
    pub async fn set_cpu_governor(&self, governor: &str) -> Result<String, zbus::fdo::Error> {
        //get cpu path
        let cpu_path = parse_yaml().unwrap().interfaces.cpu.device;

        //check if the governor is valid and set it
        if governor == "performance" || governor == "powersave" || governor == "ondemand" {
            let result = Command::new("echo")
                .arg(governor)
                .arg(format!("{}/cpufreq/scaling_governor", cpu_path))
                .output()
                .expect("Failed to execute echo command");

            if result.status.success() {
                return Ok("Governor set successfully".to_string());
            } else {
                return Err(zbus::fdo::Error::Failed(
                    "Failed to set governor".to_string(),
                ));
            }
        } else {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Invalid governor value".to_string(),
            ));
        }
    }
}

#[cfg(feature = "mock")]
#[interface(name = "Mechanix.Services.Power")]
impl PowerBusInterface {
    pub async fn get_battery_info(&self) -> Result<BatteryStatusResponse, zbus::fdo::Error> {
        // mock battery status
        let battery_status = BatteryStatusResponse {
            name: "BAT0".to_string(),
            r#type: "Li-ion".to_string(),
            status: "Discharging".to_string(),
            present: true,
            voltage_now: 12000000,
            current_now: -1000000,
            capacity: 50,
            capacity_level: "Normal".to_string(),
            temp: 25000,
            technology: "Li-ion".to_string(),
            charge_full: 10000000,
            charge_now: 5000000,
            charge_full_design: 10000000,
            manufacturer: "TexasInstruments".to_string(),
        };

        //check for error flag form config file then return error else return battery status

        if Some(true) == parse_yaml().unwrap().interfaces.battery.error {
            return Err(zbus::fdo::Error::Failed(
                "FailedToGetBatteryStatus".to_string(),
            ));
        }

        Ok(battery_status)
    }

    pub async fn get_battery_status(&self) -> Result<u8, zbus::fdo::Error> {
        // mock battery status
        let battery_status = BatteryStatusResponse {
            name: "BAT0".to_string(),
            r#type: "Li-ion".to_string(),
            status: "Discharging".to_string(),
            present: true,
            voltage_now: 12000000,
            current_now: -1000000,
            capacity: 50,
            capacity_level: "Normal".to_string(),
            temp: 25000,
            technology: "Li-ion".to_string(),
            charge_full: 10000000,
            charge_now: 5000000,
            charge_full_design: 10000000,
            manufacturer: "TexasInstruments".to_string(),
        };

        //check for error flag form config file then return error else return battery status
        if Some(true) == parse_yaml().unwrap().interfaces.battery.error {
            return Err(zbus::fdo::Error::Failed(
                "FailedToGetBatteryCapacity".to_string(),
            ));
        }

        Ok(battery_status.capacity)
    }

    pub async fn get_power_usage(&self) -> Result<i32, zbus::fdo::Error> {
        //moke power usage
        let power_usage = -1000000;

        //get power usage if  there is an error return  by default the sdk returns () we need to return a u8
        if Some(true) == parse_yaml().unwrap().interfaces.battery.error {
            return Err(zbus::fdo::Error::Failed(
                "FailedToGetPowerUsage".to_string(),
            ));
        }

        Ok(power_usage)
    }

    // get cpu governor
    pub async fn get_cpu_governor(&self) -> Result<String, zbus::fdo::Error> {
        // vector to hold mock cpu governor value then return a random value from the vector
        let cpu_governor_list = vec![
            "performance".to_string(),
            "powersave".to_string(),
            "ondemand".to_string(),
        ];

        // return random value from the vector each time the function is called

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..cpu_governor_list.len());

        // Get the corresponding value from the vector
        let cpu_governor = cpu_governor_list[random_index].to_string();
        //on success return governor or else return mock data
        if Some(true) == parse_yaml().unwrap().interfaces.cpu.error {
            return Err(zbus::fdo::Error::Failed(
                "FailedToGetCpuGovernor".to_string(),
            ));
        }

        Ok(cpu_governor)
    }

    // set cpu governor
    pub async fn set_cpu_governor(&self, governor: &str) -> Result<String, zbus::fdo::Error> {
        //get cpu path
        let cpu_path = parse_yaml().unwrap().interfaces.cpu.device;

        //if error flag is set return error else return success
        if Some(true) == parse_yaml().unwrap().interfaces.cpu.error {
            return Err(zbus::fdo::Error::Failed(
                "FailedToSetCpuGovernor".to_string(),
            ));
        }

        //check if the governor is valid and set it
        if governor == "performance" || governor == "powersave" || governor == "ondemand" {
            let result = Command::new("echo")
                .arg(governor)
                .arg(format!("{}/cpufreq/scaling_governor", cpu_path))
                .output()
                .expect("Failed to execute echo command");

            if result.status.success() {
                return Ok("Governor set successfully".to_string());
            } else {
                return Err(zbus::fdo::Error::Failed(
                    "Failed to set governor".to_string(),
                ));
            }
        } else {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Invalid governor value".to_string(),
            ));
        }
    }
}
