use mecha_display_ctl::{DisplayError, DisplayErrorCodes};
use zbus::fdo::Error as DBusError;
use anyhow::Error;

pub fn handle_display_error(e: Error) -> DBusError {
    if let Some(custom_error) = e.downcast_ref::<DisplayError>() {
        match custom_error.code {
            DisplayErrorCodes::InvalidBrightnessValueError => DBusError::Failed("InvalidBrightnessValueError".to_string()),
            DisplayErrorCodes::InvalidBrightnessPathError => DBusError::IOError("InvalidBrightnessPathError".to_string()),
        }
    } else {
        DBusError::Failed("Unknown".to_string())
    }
}
