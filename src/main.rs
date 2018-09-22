#![feature(type_ascription)]

use::std::io;

const CELSIUS_BOILING_POINT : f64 = 100.0;
const FAHRENHEIT_BOILING_POINT : f64 = 212.0;
const CELSIUS_FREEZING_POINT : f64 = 0.0;
const FAHRENHEIT_FREEZING_POINT : f64 = 32.0;


fn main() {

    menu();

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    let choice : u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
        
        match choice {

             2 => {

                println!("Enter Fahrenheit temperature: ");

                let mut f_temp = String::new();

                io::stdin().read_line(&mut f_temp)
                    .expect("Failed to read temp.");
                
                let f_temp : f64 = match f_temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0.0,
                        };

                convert_to_celsius(f_temp)
            }


            1 => {

                println!("Enter Celsius temperature: ");

                let mut c_temp = String::new();

                io::stdin().read_line(&mut c_temp)
                    .expect("Failed to read temp.");
                
                let c_temp : f64 = match c_temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0.0,
                        };

                convert_to_fahrenheit(c_temp)
            }

            3  => {
                return
            }

            _ => {
                return
            }

            }


    fn menu() {
        println!("Convert to Fahrenheit -> Press 1");
        println!("Convert to Celsius -> Press 2");
        println!("Exit program -> Press 3");
        println!("Enter choice: ");
    }
        
    fn convert_to_fahrenheit(temp : f64) {
        let f_temp  = (temp * 1.8) + 32.0;
        println!("{} degrees Celsius converts to {} degrees Fahrenheit.", temp, f_temp);
        check_temps(f_temp: f64);
    }

    fn convert_to_celsius(temp : f64) {
        let c_temp = (temp - 32.0) * 0.5556;
        println!("{} degrees Fahrenheit converts to {} degrees Celsius.", temp, c_temp);
        check_temps(c_temp :f64);
    }

    fn check_temps(temp : f64) {
       if temp == CELSIUS_BOILING_POINT {
            println!("{} degrees Celsius is the boiling point of water!", temp);
        }
        if temp == FAHRENHEIT_BOILING_POINT {
            println!("{} degrees Fahrenheit is the boiling point of water!", temp);
        }
        if temp == CELSIUS_FREEZING_POINT {
            println!("{} degrees Celsius is the freezing point of water!", temp);
        }
        if temp == FAHRENHEIT_FREEZING_POINT {
            println!("{} degrees Fahrenheit is the freezing point of water!", temp);
        }
    }

}