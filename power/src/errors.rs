use mecha_battery_ctl::{PowerSupplyError, PowerSupplyErrorCodes};
use zbus::fdo::Error as DBusError;
use anyhow::Error;

pub fn handle_power_supply_error(e: Error) -> DBusError {
    if let Some(custom_error) = e.downcast_ref::<PowerSupplyError>() {
        match custom_error.code {
            PowerSupplyErrorCodes::FailedToOpenFile => DBusError::IOError("Failed to open file".to_string()),
            PowerSupplyErrorCodes::FailedToReadFile => DBusError::IOError("Failed to read file".to_string()),
            PowerSupplyErrorCodes::InvalidDataFormat => DBusError::InvalidArgs("Invalid data format".to_string()),
            PowerSupplyErrorCodes::UnknownError => DBusError::Failed("Unknown error".to_string()),
        }
    } else {
        DBusError::Failed("Unknown error".to_string())
    }
}
