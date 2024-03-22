use zbus::interface;

use mecha_bluetooth_ctl::BluetoothControl;

pub struct BluetoothBusInterface {}

#[interface(name = "Mechanix.Services.Bluetooth")]
impl BluetoothBusInterface {
    //todo: implement the bluetooth interface methods
    pub async fn get_bluetooth_status(&self) -> bool {
        //create a new bluetooth control instance
        let bluetooth_module = BluetoothControl::new().await.unwrap();
        //get bluetooth interface status
        let status = match bluetooth_module.bluetooth_status().await {
            Ok(_) => true,
            Err(_) => false,
        };
        status
    }

    //todo: implement the bluetooth interface methods
    pub async fn get_bluetooth_info(&self) -> String {
        //get bluetooth interface info
        let info = "info".to_string();
        info
    }

    pub async fn disable_bluetooth(&self) -> bool {
        //create a new bluetooth control instance
        let bluetooth_module = BluetoothControl::new().await.unwrap();
        //disable bluetooth interface
        let status = match bluetooth_module.disable_bluetooth().await {
            Ok(_) => true,
            Err(_) => false,
        };
        status
    }

    pub async fn enable_bluetooth(&self) -> bool {
        //create a new bluetooth control instance
        let bluetooth_module = BluetoothControl::new().await.unwrap();
        //enable bluetooth interface
        let status = match bluetooth_module.enable_bluetooth().await {
            Ok(_) => true,
            Err(_) => false,
        };
        status
    }
}
