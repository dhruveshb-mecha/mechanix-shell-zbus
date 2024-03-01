use zbus::{Connection, Result, proxy};

#[proxy(
    interface = "Mechanix.Services.Display",
    default_service = "mechanix.services.display",
    default_path = "/org/mechanix/services/display"
)]
trait DisplayBusInterface {
    async fn set_display_brightness(&self, brightness: u8) -> Result<u8>;
    async fn get_display_brightness(&self) -> Result<u8>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `proxy` macro creates `MyGreeterProxy` based on `Notifications` trait.
    // let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
    // let reply = match proxy.set_display_brightness(10).await {
    //     Ok(brightness) => format!("Brightness set to: {}", brightness),
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         "Failed to set brightness".to_string()
        
    //     },
    // };



    //get display brightness
    let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
    let reply = match proxy.get_display_brightness().await {
        Ok(brightness) => format!("Display Brightness is: {}", brightness),
        Err(e) => {
            eprintln!("Error: {}", e);
            "Failed to set brightness".to_string()
        
        },
    };

    println!("{reply}");
    // println!("{reply}");

    Ok(())
}