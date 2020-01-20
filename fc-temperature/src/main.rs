use std::io;

fn main() {
    println!("FC - TEMPERATURE CONVERTER!");
    
    loop {
        println!("Choose a starting temperature: (C)elsius or (F)ahrenheit");

        let starting_temp: char = get_char_input();
        if starting_temp != 'C' && starting_temp != 'F' {
            println!(":: Input should only either be 'C' or 'F'.");
            continue;
        }

        println!("Enter temperature:");

        let temp: f64 = get_f64_input();
        match starting_temp {
            'C' => println!("In Fahrenheit: {}", ((9.0 * temp) / 5.0) + 32.0),
            'F' => println!("In Celsius: {}", ((temp - 32.0) * 5.0) / 9.0),
            _   => {
                println!(":: Input should only either be 'C' or 'F'.");
                continue;
            },
        }

        break;
    }
}

fn get_char_input() -> char {
    let mut starting_temp = String::new();
    io::stdin().read_line(&mut starting_temp)
        .expect("Failed to read line.");
    
    let starting_temp: char = match starting_temp.trim().parse() {
        Ok(temp_units) => temp_units,
        Err(_) => panic!("Wrong value!"),
    };

    starting_temp
}

fn get_f64_input() -> f64 {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp)
        .expect("Failed to read line.");
    
    let temp: f64 = match temp.trim().parse() {
        Ok(temp_val) => temp_val,
        Err(_) => panic!("Wrong value!"),
    };

    temp
}
