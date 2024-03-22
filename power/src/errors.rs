use mecha_battery_ctl::{PowerSupplyError, PowerSupplyErrorCodes};
use zbus::fdo::Error as DBusError;
use anyhow::Error;

pub fn handle_power_supply_error(e: Error) -> DBusError {
    if let Some(custom_error) = e.downcast_ref::<PowerSupplyError>() {
        match custom_error.code {
            PowerSupplyErrorCodes::FailedToOpenFile => DBusError::IOError("FailedToOpenFile".to_string()),
            PowerSupplyErrorCodes::FailedToReadFile => DBusError::IOError("FailedToReadFile".to_string()),
            PowerSupplyErrorCodes::InvalidDataFormat => DBusError::InvalidArgs("InvalidDataFormat".to_string()),
            PowerSupplyErrorCodes::UnknownError => DBusError::Failed("UnknownError".to_string()),
        }
    } else {
        DBusError::Failed("UnknownError".to_string())
    }
}
