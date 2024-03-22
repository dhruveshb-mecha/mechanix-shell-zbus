use std::future::pending;

use anyhow::Result;
use bluetooth::BluetoothBusInterface;
use device_info::DisplayInfoBusInterface;
use display::DisplayBusInterface;
use network::NetworkBusInterface;
use power::PowerBusInterface;
// use system_cmd::SystemBusInterface;
use zbus::connection;

#[tokio::main]
async fn main() -> Result<()> {
    let display_bus = DisplayBusInterface {};

    let _display_bus_connection = connection::Builder::system()?
        .name("mechanix.services.display")?
        .serve_at("/org/mechanix/services/display", display_bus)?
        .build()
        .await?;

    let power_bus = PowerBusInterface {};
    let _power_bus_connection = connection::Builder::system()?
        .name("mechanix.services.power")?
        .serve_at("/org/mechanix/services/power", power_bus)?
        .build()
        .await?;

    let network_bus = NetworkBusInterface {};
    let _network_bus_connection = connection::Builder::system()?
        .name("mechanix.services.network")?
        .serve_at("/org/mechanix/services/network", network_bus)?
        .build()
        .await?;

    let bluetooth_bus = BluetoothBusInterface {};
    let _bluetooth_bus_connection = connection::Builder::system()?
        .name("mechanix.services.bluetooth")?
        .serve_at("/org/mechanix/services/bluetooth", bluetooth_bus)?
        .build()
        .await?;

    let device_info_bus = DisplayInfoBusInterface {};
    let _device_info_bus_connection = connection::Builder::system()?
        .name("mechanix.services.deviceinfo")?
        .serve_at("/org/mechanix/services/deviceinfo", device_info_bus)?
        .build()
        .await?;

    // let system_bus = SystemBusInterface {};
    // let _system_bus_connection = connection::Builder::system()?
    //     .name("mechanix.services.system")?
    //     .serve_at("/org/mechanix/services/system", system_bus)?
    //     .build()
    //     .await?;

    //make this server run continuously
    pending::<()>().await;

    Ok(())
}
