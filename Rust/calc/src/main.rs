#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
slint::include_modules!();



fn main() -> Result<(), Box<dyn Error>> {
    
    let ui: AppWindow = AppWindow::new().unwrap();
    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();
    
    let mut vec: Vec<i32> = Vec::new();


    ui.on_button_clicked(move  |button_id: i32| {
        if let Some(ui) = ui_handle.upgrade() {
            
            let current_value: i32 = ui.get_display(); 

            match button_id {
                
                1 => { vec.push(1); }, 
                2 => { vec.push(2); },  
                3 => { vec.push(3); }, 
                4 => { vec.push(4); }, 
                5 => { vec.push(5); },  
                6 => { vec.push(6); },
                7 => { vec.push(7); }, 
                8 => { vec.push(8); }, 
                9 => { vec.push(9); }, 
                0 => { vec.push(0); },
               10 => {println!("Counter updated to: {:?}", vec_to_number(&mut vec));},  
                _ => println!("well fuck now only god knows what this code dose"),

            }
        }
    });

    ui.run()?; 
    Ok(())
}


fn vec_to_number(vec: &mut Vec<i32>) -> i32{
    
    let size: i32 = vec.len() as i32; 
    let mut number: i32 = 0;

     
    //for(int i = 0; i < size; i++)
    for i in 0..size {
        number = &number * 10 + vec[i as usize];  
    }

    return number;
}

