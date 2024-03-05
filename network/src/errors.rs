use mecha_network_ctl::errors::{WirelessNetworkError, WirelessNetworkErrorCodes};
use zbus::fdo::Error as DBusError;
use anyhow::Error;

pub fn handle_network_error(e:Error) -> DBusError {
    if let Some(custom_error) = e.downcast_ref::<WirelessNetworkError>() {
        match custom_error.code {
            WirelessNetworkErrorCodes::NoWirelessNetworkFound => DBusError::IOError("NoWirelessNetworkFound".to_string()),
            WirelessNetworkErrorCodes::UnableToTurnOnWirelessNetwork => DBusError::IOError("UnableToTurnOnWirelessNetwork".to_string()),
            WirelessNetworkErrorCodes::UnableToTurnOffWirelessNetwork => DBusError::IOError("UnableToTurnOffWirelessNetwork".to_string()),
            WirelessNetworkErrorCodes::UnableToConnectToWirelessNetwork => DBusError::Failed("UnableToConnectToWirelessNetwork".to_string()),
            WirelessNetworkErrorCodes::UnableToDisconnectWirelessNetwork => DBusError::Failed("UnableToDisconnectWirelessNetwork".to_string()),
            WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus => DBusError::NoNetwork("UnableToGetWirelessNetworkStatus".to_string()),
            WirelessNetworkErrorCodes::UnableToRemoveWirelessNetwork => DBusError::Failed("UnableToRemoveWirelessNetwork".to_string()),
            WirelessNetworkErrorCodes::Unknown => DBusError::Failed("Unknown".to_string()),
        }
    } else {
        DBusError::Failed("Unknown".to_string())
    }
}
