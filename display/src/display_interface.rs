pub use mecha_display_ctl::DisplayControl;
use utils::parse_yaml;
use zbus::interface;

use crate::handle_display_error;

//create display struct
pub struct DisplayBusInterface {}

#[interface(name = "Mechanix.Services.Display")]
impl DisplayBusInterface {
    pub async fn get_display_brightness(&self) -> Result<u8, zbus::fdo::Error> {
        //get display path
        let display_path = parse_yaml().unwrap().interfaces.display.device;

        //get display instance
        let display = DisplayControl { path: display_path };

        //get display brightness if  there is an error return  by default the sdk returns () we need to return a u8
        let result = match display.get_display_brightness() {
            Ok(brightness) => brightness,
            Err(e) => {
                return Err(handle_display_error(e));
            }
        };

        Ok(result)
    }

    pub async fn set_display_brightness(&self, brightness: u8) -> Result<u8, zbus::fdo::Error> {
        //get display path
        let display_path = parse_yaml().unwrap().interfaces.display.device;

        //get display instance
        let display = DisplayControl { path: display_path };

        //set display brightness if  there is an error return  by default the sdk returns () we need to return a u8

        let result = match display.set_display_brightness(brightness) {
            Ok(_) => brightness,
            Err(e) => {
                return Err(handle_display_error(e));
            }
        };

        Ok(result)
    }

    //set backlight on
    pub async fn turn_backlight_on(&self) -> Result<u8, zbus::fdo::Error> {
        //get display path
        let display_path = parse_yaml().unwrap().interfaces.display.device;

        //get display instance
        let display = DisplayControl { path: display_path };

        //set display brightness to maximum if  there is an error return  by default the sdk returns () we need to return a u8
        let result = match display.set_display_brightness(244) {
            Ok(_) => 244,
            Err(e) => {
                return Err(handle_display_error(e));
            }
        };

        Ok(result)
    }

    //set backlight off
    pub async fn turn_backlight_off(&self) -> Result<u8, zbus::fdo::Error> {
        //get display path
        let display_path = parse_yaml().unwrap().interfaces.display.device;

        //get display instance
        let display = DisplayControl { path: display_path };

        //set display brightness to minimum if  there is an error return  by default the sdk returns () we need to return a u8
        let result = match display.set_display_brightness(0) {
            Ok(_) => 0,
            Err(e) => {
                return Err(handle_display_error(e));
            }
        };

        Ok(result)
    }

    //todo: implement the display interface methods to get timeout
    pub async fn get_screen_timeout(&self) -> u32 {
        println!("get screen timeout");
        0
    }

    //todo: implement the display interface methods to set timeout
    pub async fn set_screen_timeout(&self, timeout: u32) -> u32 {
        println!("get screen timeout");
        10
    }
}
