use std::process::Command;
use utils::parse_yaml;

use zbus::{
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
};

pub use mecha_network_ctl::wireless_network::WirelessNetworkControl;

use crate::handle_network_error;
pub struct NetworkBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `ScanResultZbus` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct ScanResultZbus {
    pub name: String,
    pub signal: i32,
    pub frequency: String,
    pub mac: String,
    pub flags: String,
}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `ScanWirelessNetworkZbus` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct ScanWirelessNetworkZbus {
    pub name: String,
    pub signal: i32,
    pub frequency: String,
    pub mac: String,
    pub flags: String,
}
//Vector of ScanWirelessNetworkZbus
#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct ScanWirelessNetworkListZbus {
    pub networks: Vec<ScanWirelessNetworkZbus>,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct NetworkResultZbus {
    pub network_id: u32,
    pub ssid: String,
    pub flags: String,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct NetworkListZbus {
    pub networks: Vec<NetworkResultZbus>,
}

#[interface(name = "Mechanix.Services.Network")]
impl NetworkBusInterface {
    pub async fn get_wireless_interface_status(&self) -> bool {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //get wireless interface from the path
        let interface_name = wireless_network_path.split('/').last().unwrap();

        //get wireless interface status
        let output = Command::new("ifconfig")
            .output()
            .expect("Failed to execute ifconfig command");

        let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");

        let status: bool;
        // Check if the stdout contains interface_name
        if stdout.contains(interface_name) {
            status = true;
        } else {
            status = false;
        }
        status
    }

    //get wireless interface info
    pub async fn get_wireless_interface_info(&self) -> Result<ScanResultZbus, zbus::fdo::Error> {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //get wireless network instance
        let network_module = WirelessNetworkControl::new(&wireless_network_path);

        // get wireless interface info
        let result = match network_module.current_wireless_network().await {
            Ok(result) => result,
            Err(e) => {
                return Err(handle_network_error(e));
            }
        };
        Ok(ScanResultZbus {
            name: result.name,
            signal: result.signal as i32,
            frequency: result.frequency,
            mac: result.mac,
            flags: result.flags,
        })
    }

    pub async fn disable_wireless_interface(&self) -> Result<bool, zbus::fdo::Error> {
        //get wireless network path

        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //extract the interface name from the path
        let interface_name = wireless_network_path.split('/').last().unwrap();

        //disable wireless interface
        let output = Command::new("ifconfig")
            .arg(interface_name)
            .arg("down")
            .output()
            .expect("Failed to execute ifconfig command");

        if output.status.success() {
            Ok(true)
        } else {
            Err(zbus::fdo::Error::Failed(
                "Failed to disable wireless interface".to_string(),
            ))
        }
    }

    pub async fn enable_wireless_interface(&self) -> Result<bool, zbus::fdo::Error> {
        //get wireless network path

        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //extract the interface name from the path
        let interface_name = wireless_network_path.split('/').last().unwrap();
        //disable wireless interface

        let output = Command::new("ifconfig")
            .arg(interface_name)
            .arg("up")
            .output()
            .expect("Failed to execute ifconfig command");

        if output.status.success() {
            Ok(true)
        } else {
            Err(zbus::fdo::Error::Failed(
                "Failed to disable wireless interface".to_string(),
            ))
        }
    }

    // todo: implement connect_to_wireless_network will be same as add network method but works only for known networks
    // pub async fn connect_to_wireless_network(&self,network_id: u32) -> bool {
    //     //connect to wireless network

    // }

    pub async fn scan_wireless_networks(
        &self,
    ) -> Result<ScanWirelessNetworkListZbus, zbus::fdo::Error> {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //get wireless network instance
        let network_module = WirelessNetworkControl::new(&wireless_network_path);

        //scan wireless networks
        let result = match network_module.scan_wireless_network().await {
            Ok(scan_results) => scan_results,
            Err(e) => {
                return Err(handle_network_error(e));
            }
        };

        Ok(ScanWirelessNetworkListZbus {
            networks: result
                .iter()
                .map(|network| ScanWirelessNetworkZbus {
                    name: network.name.clone(),
                    signal: network.signal as i32,
                    frequency: network.frequency.clone(),
                    mac: network.mac.clone(),
                    flags: network.flags.clone(),
                })
                .collect::<Vec<ScanWirelessNetworkZbus>>(),
        })
    }

    pub async fn add_wireless_network(
        &self,
        ssid: String,
        psk: String,
    ) -> Result<bool, zbus::fdo::Error> {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //call connect_wireless_network method if successful return true else return false
        let result = match WirelessNetworkControl::connect_wireless_network(
            &wireless_network_path,
            &ssid,
            &psk,
        )
        .await
        {
            Ok(_) => true,
            Err(e) => {
                return Err(handle_network_error(e));
            }
        };

        Ok(result)
    }

    pub async fn remove_wireless_network(&self, network_id: u32) -> Result<bool, zbus::fdo::Error> {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //call remove_wireless_network method if successful return true else return false

        let result = match WirelessNetworkControl::remove_wireless_network(
            &wireless_network_path,
            network_id as usize,
        )
        .await
        {
            Ok(_) => true,
            Err(e) => {
                return Err(handle_network_error(e));
            }
        };

        Ok(result)
    }

    //todo: implement update_wireless_network
    pub async fn update_wireless_network(&self) -> String {
        //update wireless network
        let status = "updated".to_string();
        status
    }

    pub async fn get_wireless_networks(&self) -> Result<NetworkListZbus, zbus::fdo::Error> {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //get wireless network list

        let result =
            match WirelessNetworkControl::get_known_wireless_networks(&wireless_network_path).await
            {
                Ok(networks) => networks,
                Err(e) => {
                    return Err(handle_network_error(e));
                }
            };

        Ok(NetworkListZbus {
            networks: result
                .iter()
                .map(|network| NetworkResultZbus {
                    network_id: network.network_id as u32,
                    ssid: network.ssid.clone(),
                    flags: network.flags.clone(),
                })
                .collect::<Vec<NetworkResultZbus>>(),
        })
    }
}
