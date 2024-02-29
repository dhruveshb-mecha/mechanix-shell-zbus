use mecha_battery_ctl::{Battery, BatteryControl, PowerSupplyInfo};
use zbus::interface;

pub struct PowerBusInterface {}

#[interface(name = "Mechanix.Services.Power")]
impl PowerBusInterface {
    /*
    Mechanix.Services.Power.GetBatteryStatus
    Mechanix.Services.Power.GetBatteryInfo
    Mechanix.Services.Power.GetPowerUsage
    Mechanix.Services.Power.SetCPUGovernor
    Mechanix.Services.Power.GetCPUGovernorInfo
    */

    pub async fn get_battery_status(&self) -> String {
        //get battery instance
        let battery = Battery {
            path: "/sys/class/power_supply/BAT0".to_string(),
            currnet_now: "/sys/class/power_supply/BAT0/current_now".to_string(),
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
}
