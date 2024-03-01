use mecha_battery_ctl::{Battery, BatteryControl, PowerSupplyInfo};
use utils::parse_yaml;
use zbus::interface;

pub struct PowerBusInterface {}

#[interface(name = "Mechanix.Services.Power")]
impl PowerBusInterface {
    pub async fn get_battery_status(&self) -> String {
        //get battery path
        let battery_path = parse_yaml().unwrap().interfaces.battery.device;

        //get battery instance
        let battery = Battery {
            path: format!("{}/uevent", battery_path),
            currnet_now: format!("{}/current_now", battery_path),
        };

        //moke PowerSupplyInfo object
        let power_supply_info = BatteryControl {
            name: "BAT0".to_string(),
            r#type: "Battery".to_string(),
            status: "Discharging".to_string(),
            present: true,
            voltage_now: 12000,
            current_now: -1000,
            capacity: 80,
            capacity_level: "High".to_string(),
            temp: 30,
            technology: "Li-ion".to_string(),
            charge_full: 10000,
            charge_now: 8000,
            charge_full_design: 10000,
            manufacturer: "LG".to_string(),
        };

        //get battery status if  there is an error return moke BatteryInfo
        let result = match battery.info() {
            Ok(status) => status,
            Err(_) => power_supply_info,
        };

        result.status
    }

    //get battery percentage
    pub async fn get_battery_info(&self) -> u8 {
        //get battery path
        let battery_path = parse_yaml().unwrap().interfaces.battery.device;

        //get battery instance
        let battery = Battery {
            path: format!("{}/uevent", battery_path),
            currnet_now: format!("{}/current_now", battery_path),
        };

        //get battery percentage if  there is an error return  by default the sdk returns () we need to return a u8
        let result: u8 = match battery.info() {
            Ok(battery_info) => battery_info.capacity,
            Err(_) => 45,
        };

        result
    }

    //get power usage
    pub async fn get_power_usage(&self) -> i32 {
        //get battery path
        let battery_path = parse_yaml().unwrap().interfaces.battery.device;

        //get battery instance
        let battery = Battery {
            path: format!("{}/uevent", battery_path),
            currnet_now: format!("{}/current_now", battery_path),
        };

        //get power usage if  there is an error return  by default the sdk returns () we need to return a u8
        let result: i32 = match battery.info() {
            Ok(battery_info) => battery_info.current_now,
            Err(_) => 45,
        };

        result
    }
}
