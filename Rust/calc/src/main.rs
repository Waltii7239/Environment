#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
slint::include_modules!();



fn main() -> Result<(), Box<dyn Error>> {
    
    let ui: AppWindow = AppWindow::new().unwrap();
    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();
    
    let mut vec: Vec<i32> = Vec::new();
    let mut opctr: i8 = 0; 
    let mut nummber_1: i32 = 0;
    let mut nummber_2: i32 = 9;

    let add  = |num1: i32, num2: i32| num1 + num2;
    let sub  = |num1: i32, num2: i32| num1 - num2;
    let mult = |num1: i32, num2: i32| num1 * num2;
    let div  = |num1: i32, num2: i32| num1 / num2;




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

                11 => {
                    vec.clear();
                    nummber_1 = 0;
                    nummber_2 = 0;
                    opctr = 0;
                    ui.set_display(0)
                }, // AC 

                12 => { 
                    opctr = 1;
                    if nummber_1 == 0{nummber_1 = vec_to_number(&vec);}
                    vec.clear();
                }, // -

                13 => { 
                    opctr = 2;
                    if nummber_1 == 0{nummber_1 = vec_to_number(&vec);}
                    vec.clear();
                }, // +
                14 => { 
                    opctr = 3;
                    if nummber_1 == 0{nummber_1 = vec_to_number(&vec);}
                    vec.clear()
                },
                15 => { 
                    opctr = 4;
                    if nummber_1 == 0{nummber_1 = vec_to_number(&vec);}
                    vec.clear()
                },


               10 => {
                    match opctr{
                        1 => {
                             nummber_2 = vec_to_number(&vec);
                            ui.set_display(sub(nummber_1, nummber_2));
                            nummber_1 = sub(nummber_1, nummber_2)

                        }
                        2 => {
                            nummber_2 = vec_to_number(&vec);
                            ui.set_display(add(nummber_1, nummber_2));
                            nummber_1 = add(nummber_1, nummber_2)
                        },
                        3 => {
                            nummber_2 = vec_to_number(&vec);
                            ui.set_display(mult(nummber_1, nummber_2));
                            nummber_1 = mult(nummber_1, nummber_2)
                        },
                        4 => {
                            nummber_2 = vec_to_number(&vec);
                            ui.set_display(div(nummber_1, nummber_2));
                            nummber_1 = div(nummber_1, nummber_2)
                        },

                        
                        _ => {
                            println!("Err")
                        }, 
                    }
                    println!("Counter updated to: {:?}", vec_to_number(&mut vec));
                    println!("{:?}", opctr);
                },

                _ => println!("well fuck now only god knows what this code dose"),

            }
        }
    });

    ui.run()?; 
    Ok(())
}


fn vec_to_number(vec: &Vec<i32>) -> i32{
    
    let size: i32 = vec.len() as i32; 
    let mut number: i32 = 0;

     
    //for(int i = 0; i < size; i++)
    for i in 0..size {
        number = &number * 10 + vec[i as usize];  
    }

    return number;
}

