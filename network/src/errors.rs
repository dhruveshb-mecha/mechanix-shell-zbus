use mecha_network_ctl::errors::{WirelessNetworkError, WirelessNetworkErrorCodes};
use zbus::fdo::Error as DBusError;
use anyhow::Error;

pub fn handle_network_error(e:Error) -> DBusError {
    if let Some(custom_error) = e.downcast_ref::<WirelessNetworkError>() {
        match custom_error.code {
            WirelessNetworkErrorCodes::NoWirelessNetworkFound => DBusError::IOError("No wireless network found".to_string()),
            WirelessNetworkErrorCodes::UnableToTurnOnWirelessNetwork => DBusError::IOError("Unable to turn on wireless network".to_string()),
            WirelessNetworkErrorCodes::UnableToTurnOffWirelessNetwork => DBusError::IOError("Unable to turn off wireless network".to_string()),
            WirelessNetworkErrorCodes::UnableToConnectToWirelessNetwork => DBusError::Failed("Unable to connect to wireless network".to_string()),
            WirelessNetworkErrorCodes::UnableToDisconnectWirelessNetwork => DBusError::Failed("Unable to disconnect from wireless network".to_string()),
            WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus => DBusError::NoNetwork("Unable to get wireless network status".to_string()),
            WirelessNetworkErrorCodes::UnableToRemoveWirelessNetwork => DBusError::Failed("Unable to remove wireless network".to_string()),
            WirelessNetworkErrorCodes::Unknown => DBusError::Failed("Unknown error".to_string()),
        }
    } else {
        DBusError::Failed("Unknown error".to_string())
    }
}
