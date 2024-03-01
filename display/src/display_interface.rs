pub use mecha_display_ctl::DisplayControl;
use utils::parse_yaml;
use zbus::interface;

//create display struct
pub struct DisplayBusInterface {}

#[interface(name = "Mechanix.Services.Display")]
impl DisplayBusInterface {
    pub async fn get_display_brightness(&self) -> u8 {
        //get display path
        let display_path = parse_yaml().unwrap().interfaces.display.device;

        //get display instance
        let display = DisplayControl { path: display_path };

        //get display brightness if  there is an error return  by default the sdk returns () we need to return a u8
        let result = match display.get_display_brightness() {
            Ok(brightness) => brightness,
            Err(_) => 45,
        };

        result
    }

    pub async fn set_display_brightness(&self, brightness: u8) -> u8 {
        //get display path
        let display_path = parse_yaml().unwrap().interfaces.display.device;

        //get display instance
        let display = DisplayControl { path: display_path };

        //set display brightness if  there is an error return  by default the sdk returns () we need to return a u8

        let result = match display.set_display_brightness(brightness) {
            Ok(_) => brightness,
            Err(_) => 45,
        };

        result
    }
}
