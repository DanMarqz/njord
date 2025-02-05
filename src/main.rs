// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod crypto_price;
mod time;

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;


    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            match crypto_price::get_bitcoin_price() {
                Ok(price) => {
                    let now = time::get_time();
                    ui.set_time(slint::SharedString::from(now).clone());

                    let price = format!("{:.2}", price); // Will be return 1.23
                    ui.set_price(slint::SharedString::from(price).clone());
                }
                Err(_) => {
                    println!("Failed to grab price");
                }
            }
        }
    });

    ui.run()?;

    Ok(())
}
