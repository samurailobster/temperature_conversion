#![feature(type_ascription)]

use::std::io;

const CELSIUS_BOILING_POINT : f64 = 100.0;
const FAHRENHEIT_BOILING_POINT : f64 = 212.0;
const CELSIUS_FREEZING_POINT : f64 = 0.0;
const FAHRENHEIT_FREEZING_POINT : f64 = 32.0;


fn main() {

    loop{
    menu();

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    let choice : u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
        
    match choice {
        1 => {
            println!("Enter Celsius temperature: ");
            let temp = get_temp();
            convert_to_fahrenheit(temp);
            }
        2 => {
            println!("Enter Fahrenheit temperature: ");
            let temp = get_temp();
            convert_to_celsius(temp);
            }
        3  => {
                return
            }
        _ => {
                return
            }
}

fn get_temp() -> f64 {

    let mut temp = String::new();

    io::stdin().read_line(&mut temp)
        .expect("Failed to read temp.");
    
    let temp : f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
            };
    
    temp
}

fn menu() {
        println!("Convert to Fahrenheit -> Press 1");
        println!("Convert to Celsius -> Press 2");
        println!("Exit program -> Press 3");
        println!("Enter choice: ");
    }
        
fn convert_to_fahrenheit(temp : f64) {
    let f_temp  = (temp * 1.8) + 32.0;
    println!("{} °C converts to {} °F.", temp, f_temp);
    check_temps(f_temp: f64);
    }

fn convert_to_celsius(temp : f64) {
    let c_temp = (temp - 32.0) * 0.5556;
    println!("{} °F converts to {} °C.", temp, c_temp);
    check_temps(c_temp :f64);
    }

fn check_temps(temp : f64) {
    if temp == CELSIUS_BOILING_POINT {
        println!("{} °C is the boiling point of water!", temp);
        }
    if temp == FAHRENHEIT_BOILING_POINT {
        println!("{} °F is the boiling point of water!", temp);
        }
    if temp == CELSIUS_FREEZING_POINT {
        println!("{} °C is the freezing point of water!", temp);
        }
    if temp == FAHRENHEIT_FREEZING_POINT {
        println!("{} °F is the freezing point of water!", temp);
        }
    if temp >= (300.0 * CELSIUS_BOILING_POINT) {
        println!("{} °C is equal to the temperature of lightning!", temp);
        }
    }

}
}