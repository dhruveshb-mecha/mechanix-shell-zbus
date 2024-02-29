use std::future::pending;

use anyhow::Result;
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

    //make this server run continuously
    pending::<()>().await;

    Ok(())

}