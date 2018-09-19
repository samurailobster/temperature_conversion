use::std::io;

fn main() {

    menu();

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim();

        match choice {

            "F" | "f" => {

                println!("Enter Fahrenheit temperature: ");

                let mut f_temp = String::new();

                io::stdin().read_line(&mut f_temp)
                    .expect("Failed to read temp.");
                
                let f_temp : f32 = match f_temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0.0,
                        };

                convert_to_celsius(f_temp)
            }


            "C" | "c" => {

                println!("Enter Celsius temperature: ");

                let mut c_temp = String::new();

                io::stdin().read_line(&mut c_temp)
                    .expect("Failed to read temp.");
                
                let c_temp : f32 = match c_temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0.0,
                        };

                convert_to_fahrenheit(c_temp)
            }

            &_ => {
                return
            }

            }


    fn menu() {
        println!("Convert to Fahrenheit -> Press F");
        println!("Convert to Celsius -> Press C");
        println!("Exit program -> Press E");
        println!("Enter choice: ");
    }



        
    fn convert_to_fahrenheit(temp : f32) {
        let f_temp  = (temp * 1.8) + 32.0;
        println!("{} degrees Celsius converts to {} degrees Fahrenheit.\n", temp, f_temp);
    }

    fn convert_to_celsius(temp : f32) {
        let c_temp = (temp - 32.0) * 0.5556;
        println!("{} degrees Fahrenheit converts to {} degrees Celsius.\n", temp, c_temp);
    }

}