use std::future::pending;

use anyhow::Result;
use bluetooth::BluetoothBusInterface;
use network::NetworkBusInterface;
use zbus::connection;
use display::DisplayBusInterface;
use power::PowerBusInterface;

#[tokio::main]
async fn main() -> Result<()> {

    let display_bus = DisplayBusInterface{};

    let _display_bus_connection =  connection::Builder::session()?
    .name("mechanix.services.display")?
            .serve_at("/org/mechanix/services/display", display_bus)?
    .build()
    .await?;


    let power_bus = PowerBusInterface{};
    let _power_bus_connection = connection::Builder::session()?
    .name("mechanix.services.power")?
    .serve_at("/org/mechanix/services/power", power_bus)?
    .build()
    .await?;


    let network_bus = NetworkBusInterface{};
    let _network_bus_connection = connection::Builder::session()?
    .name("mechanix.services.network")?
    .serve_at("/org/mechanix/services/network", network_bus)?
    .build()
    .await?;

    
    let bluetooth_bus = BluetoothBusInterface{};
    let _bluetooth_bus_connection = connection::Builder::session()?
    .name("mechanix.services.bluetooth")?
    .serve_at("/org/mechanix/services/bluetooth", bluetooth_bus)?
    .build()
    .await?;

    //make this server run continuously
    pending::<()>().await;

    Ok(())

}