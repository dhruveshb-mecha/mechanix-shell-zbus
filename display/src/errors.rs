use mecha_display_ctl::{DisplayError, DisplayErrorCodes};
use zbus::fdo::Error as DBusError;
use anyhow::Error;

pub fn handle_display_error(e: Error) -> DBusError {
    if let Some(custom_error) = e.downcast_ref::<DisplayError>() {
        match custom_error.code {
            DisplayErrorCodes::InvalidBrightnessValueError => DBusError::Failed("Invalid brightness value".to_string()),
            DisplayErrorCodes::InvalidBrightnessPathError => DBusError::IOError("Invalid brightness path".to_string()),
        }
    } else {
        DBusError::Failed("Unknown error".to_string())
    }
}
