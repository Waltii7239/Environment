#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new().unwrap();

    let ui_handle = ui.as_weak();

    ui.on_button_clicked(move || {
        if let Some(ui) = ui_handle.upgrade() {
            let current_value = ui.get_counter(); 
            ui.set_counter(current_value + 1);   
            println!("Counter updated to: {:?}", current_value + 1);
        }
    });

    ui.run()?; 

    Ok(())
}

