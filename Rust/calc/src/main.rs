#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui: AppWindow = AppWindow::new().unwrap();

    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();

    ui.on_button_clicked(move  |button_id: i32| {
        if let Some(ui) = ui_handle.upgrade() {
            
            let current_value: i32 = ui.get_display(); 

            match button_id {
                
                
                2 => ui.set_display(current_value + 2),
                3 => ui.set_display(current_value + 3),
                4 => ui.set_display(current_value + 4),
                5 => ui.set_display(current_value + 5),
                6 => ui.set_display(current_value + 6),
                7 => ui.set_display(current_value + 7),
                8 => ui.set_display(current_value + 8),
                9 => ui.set_display(current_value + 9),
                0 => ui.set_display(current_value + 0),
                _  => println!("well fuck now only god knows what this code dose"),

            }
             
            println!("Counter updated to: {:?}", current_value + 1);
        }
    });

    ui.run()?; 

    Ok(())
}

