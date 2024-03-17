use zbus::{
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
};

pub struct DisplayInfoBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct DistroInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub version_id: String,
    pub pretty_name: String,
    pub distro_codename: String,
    pub mac_address: String,
}

#[interface(name = "Mechanix.Services.DeviceInfo")]
impl DisplayInfoBusInterface {
    pub async fn get_distro_info(&self) -> Result<DistroInfo, zbus::fdo::Error> {
        //get distro info
        let distro_info = DistroInfo {
            id: "1".to_string(),
            name: "Ubuntu".to_string(),
            version: "20.04".to_string(),
            version_id: "20.04".to_string(),
            pretty_name: "Ubuntu 20.04.2 LTS".to_string(),
            distro_codename: "focal".to_string(),
            mac_address: "00:00:00:00:00:00".to_string(),
        };

        Ok(distro_info)
    }
}
