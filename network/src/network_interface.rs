use std::process::Command;

use utils::parse_yaml;

use zbus::interface;

pub use mecha_network_ctl::wireless_network::WirelessNetworkControl;

pub struct NetworkBusInterface {}

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
    pub async fn get_wireless_interface_info(&self) -> (String, i32, String, String, String) {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;


        //get wireless network instance
        let network_module = WirelessNetworkControl::new(&wireless_network_path);

        //get wireless interface info
        let result = match network_module.current_wireless_network().await {
            Ok(info) => info,
            Err(e) => {
                eprintln!("Error: {}", e);
                //return dummy info
                wifi_ctrl::sta::ScanResult {
                    name: "Network 1".to_string(),
                    signal: 1,
                    frequency: "2.4".to_string(),
                    mac: "00:00:00:00:00:00".to_string(),
                    flags: "WPA2-PSK".to_string(),
                }
            }
        };

        (result.name, result.signal as i32, result.frequency, result.mac, result.flags)

    }

    pub async fn disable_wireless_interface(&self) -> bool {
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

        output.status.success()
    }

    pub async fn enable_wireless_interface(&self) -> bool {
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

        output.status.success()
    }

    // todo: implement connect_to_wireless_network will be same as add network method but works only for known networks
    // pub async fn connect_to_wireless_network(&self,network_id: u32) -> bool {
    //     //connect to wireless network

    // }

    pub async fn scan_wireless_networks(&self) -> Vec<(String, i32, String)> {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;



        //get wireless network instance
        let network_module = WirelessNetworkControl::new(&wireless_network_path);

        //scan wireless networks
        let result = match network_module.scan_wireless_network().await {
            Ok(scan_results) => scan_results,
            Err(e) => {
                eprintln!("Error: {}", e);
                //return vector of  dummy scan results
                vec![
                    wifi_ctrl::sta::ScanResult {
                        name: "Network 1".to_string(),
                        signal: 1,
                        frequency: "2.4".to_string(),
                        mac: "00:00:00:00:00:00".to_string(),
                        flags: "WPA2-PSK".to_string(),
                    },
                    wifi_ctrl::sta::ScanResult {
                        name: "Network 2".to_string(),
                        signal: 2,
                        frequency: "5.0".to_string(),
                        mac: "00:00:00:00:00:00".to_string(),
                        flags: "WPA2-PSK".to_string(),
                    },
                ]
            }
        };

        result
            .iter()
            .map(|network| {
                (
                    network.name.clone(),
                    network.signal as i32,
                    network.frequency.clone(),
                )
            })
            .collect::<Vec<(String, i32, String)>>()
    }

    pub async fn add_wireless_network(&self, ssid: String, psk: String) -> bool {
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
                eprintln!("Error: {}", e);
                false
            }
        };

        result
    }

    pub async fn remove_wireless_network(&self, network_id: u32) -> bool {
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
                eprintln!("Error: {}", e);
                false
            }
        };

        result
    }

    //todo: implement update_wireless_network
    pub async fn update_wireless_network(&self) -> String {
        //update wireless network
        let status = "updated".to_string();
        status
    }

    pub async fn get_wireless_networks(&self) -> Vec<(u32, String, String)> {
        //get wireless network path
        let wireless_network_path = parse_yaml().unwrap().interfaces.network.device;

        //get wireless network list

        let result =
            match WirelessNetworkControl::get_known_wireless_networks(&wireless_network_path).await
            {
                Ok(networks) => networks,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    //return vector of  dummy scan results
                    vec![
                        wifi_ctrl::sta::NetworkResult {
                            network_id: 1,
                            ssid: "Network 1".to_string(),
                            flags: "2.5 ghz".to_string(),
                        },
                        wifi_ctrl::sta::NetworkResult {
                            network_id: 1,
                            ssid: "Network 2".to_string(),
                            flags: "2.5 ghz".to_string(),
                        },
                    ]
                }
            };

        result
            .iter()
            .map(|network| {
                (
                    network.network_id as u32,
                    network.ssid.clone(),
                    network.flags.clone(),
                )
            })
            .collect::<Vec<(u32, String, String)>>()
    }
}
