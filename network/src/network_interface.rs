use std::process::Command;
use utils::parse_yaml;

use zbus::{
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    Connection,
};
use zbus_polkit::policykit1::*;

pub use mecha_network_ctl::wireless_network::WirelessNetworkControl;

use crate::handle_network_error;
pub struct NetworkBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `ScanResultResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct ScanResultResponse {
    pub name: String,
    pub signal: i32,
    pub frequency: String,
    pub mac: String,
    pub flags: String,
}

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `ScanWirelessNetworkResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct ScanWirelessNetworkResponse {
    pub name: String,
    pub signal: i32,
    pub frequency: String,
    pub mac: String,
    pub flags: String,
}
//Vector of ScanWirelessNetworkListResponse
#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct ScanWirelessNetworkListResponse {
    pub networks: Vec<ScanWirelessNetworkResponse>,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct NetworkResultResponse {
    pub network_id: u32,
    pub ssid: String,
    pub flags: String,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct NetworkListResponse {
    pub networks: Vec<NetworkResultResponse>,
}
#[cfg(not(feature = "mock"))]
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
    pub async fn get_wireless_interface_info(
        &self,
    ) -> Result<ScanResultResponse, zbus::fdo::Error> {
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
        Ok(ScanResultResponse {
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
    ) -> Result<ScanWirelessNetworkListResponse, zbus::fdo::Error> {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //if authorized scan wireless networks
        if authorized().await.unwrap() {
            //get wireless network instance
            println!("Wireless network path: {}", wireless_network_path);
            let network_module = WirelessNetworkControl::new(&wireless_network_path);

            let result = match network_module.scan_wireless_network().await {
                Ok(scan_results) => scan_results,
                Err(e) => {
                    return Err(handle_network_error(e));
                }
            };

            Ok(ScanWirelessNetworkListResponse {
                networks: result
                    .iter()
                    .map(|network| ScanWirelessNetworkResponse {
                        name: network.name.clone(),
                        signal: network.signal as i32,
                        frequency: network.frequency.clone(),
                        mac: network.mac.clone(),
                        flags: network.flags.clone(),
                    })
                    .collect::<Vec<ScanWirelessNetworkResponse>>(),
            })
        } else {
            Err(zbus::fdo::Error::Failed(
                "NotAuthorizedToScanWirelessNetworks".to_string(),
            ))
        }
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

    pub async fn get_wireless_networks(&self) -> Result<NetworkListResponse, zbus::fdo::Error> {
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

        Ok(NetworkListResponse {
            networks: result
                .iter()
                .map(|network| NetworkResultResponse {
                    network_id: network.network_id as u32,
                    ssid: network.ssid.clone(),
                    flags: network.flags.clone(),
                })
                .collect::<Vec<NetworkResultResponse>>(),
        })
    }
}

#[cfg(feature = "mock")]
#[interface(name = "Mechanix.Services.Network")]
impl NetworkBusInterface {
    pub async fn get_wireless_interface_status(&self) -> Result<bool, zbus::fdo::Error> {
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

        Ok(status)
    }

    //get wireless interface info
    pub async fn get_wireless_interface_info(
        &self,
    ) -> Result<ScanResultResponse, zbus::fdo::Error> {
        //mock ScanResultResponse
        let scan_response_result = ScanResultResponse {
            name: "Mecha_Network".to_string(),
            signal: 100,
            frequency: "2.4GHz".to_string(),
            mac: "00:00:00:00:00:00".to_string(),
            flags: "WPA2".to_string(),
        };

        if let Some(true) = parse_yaml().unwrap().interfaces.network.error {
            return Err(zbus::fdo::Error::Failed(
                "NoWirelessNetworkFound".to_string(),
            ));
        }

        Ok(scan_response_result)
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
            //check for error flag from config
            if let Some(true) = parse_yaml().unwrap().interfaces.network.error {
                return Err(zbus::fdo::Error::Failed(
                    "UnableToTurnOffWirelessNetwork".to_string(),
                ));
            }
            return Ok(true);
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
            //check for error flag from config
            if let Some(true) = parse_yaml().unwrap().interfaces.network.error {
                return Err(zbus::fdo::Error::Failed(
                    "UnableToTurnOnWirelessNetwork".to_string(),
                ));
            }
            return Ok(true);
        }
    }

    // todo: implement connect_to_wireless_network will be same as add network method but works only for known networks
    // pub async fn connect_to_wireless_network(&self,network_id: u32) -> bool {
    //     //connect to wireless network

    // }

    pub async fn scan_wireless_networks(
        &self,
    ) -> Result<ScanWirelessNetworkListResponse, zbus::fdo::Error> {
        //mock Vec of ScanResult
        let scan_results = vec![
            ScanResultResponse {
                name: "Mecha_Network".to_string(),
                signal: 100,
                frequency: "2.4GHz".to_string(),
                mac: "00:00:00:00:00:00".to_string(),
                flags: "WPA2".to_string(),
            },
            ScanResultResponse {
                name: "Mecha_Network_2".to_string(),
                signal: 70,
                frequency: "5.0GHz".to_string(),
                mac: "00:00:00:00:00:00".to_string(),
                flags: "WPA2".to_string(),
            },
        ];

        //if error flag is true then return error
        if let Some(true) = parse_yaml().unwrap().interfaces.network.error {
            return Err(zbus::fdo::Error::Failed(
                "NoWirelessNetworkFound".to_string(),
            ));
        }

        Ok(ScanWirelessNetworkListResponse {
            networks: scan_results
                .iter()
                .map(|network| ScanWirelessNetworkResponse {
                    name: network.name.clone(),
                    signal: network.signal as i32,
                    frequency: network.frequency.clone(),
                    mac: network.mac.clone(),
                    flags: network.flags.clone(),
                })
                .collect::<Vec<ScanWirelessNetworkResponse>>(),
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
            Err(_e) => {
                //check for error flag from config
                if let Some(true) = parse_yaml().unwrap().interfaces.network.error {
                    return Err(zbus::fdo::Error::Failed(
                        "UnableToAddWirelessNetwork".to_string(),
                    ));
                }
                return Ok(true);
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
            Err(_e) => {
                if let Some(true) = parse_yaml().unwrap().interfaces.network.error {
                    return Err(zbus::fdo::Error::Failed(
                        "UnableToRemoveWirelessNetwork".to_string(),
                    ));
                }
                return Ok(false);
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

    pub async fn get_wireless_networks(&self) -> Result<NetworkListResponse, zbus::fdo::Error> {
        //moke Vec! NetworkResult
        let network_result = vec![
            NetworkResultResponse {
                network_id: 1,
                ssid: "Mecha_Network".to_string(),
                flags: "WPA2/PSK".to_string(),
            },
            NetworkResultResponse {
                network_id: 2,
                ssid: "Mecha_Network_2".to_string(),
                flags: "WPA".to_string(),
            },
            NetworkResultResponse {
                network_id: 3,
                ssid: "Mecha_Network_3".to_string(),
                flags: "WPA2".to_string(),
            },
        ];

        //if error flag is true then return error
        if let Some(true) = parse_yaml().unwrap().interfaces.network.error {
            return Err(zbus::fdo::Error::Failed(
                "UnableToGetWirelessNetwork".to_string(),
            ));
        }

        Ok(NetworkListResponse {
            networks: network_result
                .iter()
                .map(|network| NetworkResultResponse {
                    network_id: network.network_id as u32,
                    ssid: network.ssid.clone(),
                    flags: network.flags.clone(),
                })
                .collect::<Vec<NetworkResultResponse>>(),
        })
    }
}

async fn authorized() -> Result<bool, Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;
    let proxy = AuthorityProxy::new(&connection).await?;
    let subject = Subject::new_for_owner(std::process::id(), None, None)?;
    let result = proxy
        .check_authorization(
            &subject,
            "Mechanix.Services.Network",
            &std::collections::HashMap::new(),
            CheckAuthorizationFlags::AllowUserInteraction.into(),
            "",
        )
        .await?;
    Ok(result.is_authorized)
}
