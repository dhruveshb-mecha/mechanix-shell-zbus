use zbus::{
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
};

pub struct DeviceInfoBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct DeviceInfo {
    pub os_name: String,
    pub os_version: String,
    pub serial_number: String,
    pub wifi_mac_address: String,
    pub ethernet_mac_address: String,
}

#[interface(name = "Mechanix.Services.DeviceInfo")]
impl DeviceInfoBusInterface {
    pub async fn get_distro_info(&self) -> Result<DeviceInfo, zbus::fdo::Error> {
        //get distro info
        let distro_info = DeviceInfo {
            os_name: "Ubuntu".to_string(),
            os_version: "20.04".to_string(),
            serial_number: "1234 4654 456".to_string(),
            wifi_mac_address: "00:00:00:00:00:00".to_string(),
            ethernet_mac_address: "00:00:00:00:00:00".to_string(),
        };

        Ok(distro_info)
    }
}
