use std::io;

#[derive(Debug)]
enum TemperatureUnit {
    C,
    F,
    K,
}

// Parses the numerical value from the input string as f64 and returns it.
fn parse_value(input: &str) -> Option<f64> {
    match input.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

// Parses the temperature unit from the input string as TemperatureUnit and returns it.
fn parse_unit(input: &str) -> Option<TemperatureUnit> {
    match input.trim().to_lowercase().as_str() {
        "c" => Some(TemperatureUnit::C),
        "f" => Some(TemperatureUnit::F),
        "k" => Some(TemperatureUnit::K),
        _ => None,
    }
}

// Converts the input temperature value from one unit to the other, and returns it.
fn convert(input: f64, from_unit: &TemperatureUnit, to_unit: &TemperatureUnit) -> f64 {
    match from_unit {
        TemperatureUnit::C => match to_unit {
            TemperatureUnit::C => input,
            TemperatureUnit::F => input * 9.0 / 5.0 + 32.0,
            TemperatureUnit::K => input + 273.15,
        },
        TemperatureUnit::F => match to_unit {
            TemperatureUnit::C => (input - 32.0) * 5.0 / 9.0,
            TemperatureUnit::F => input,
            TemperatureUnit::K => (input + 459.67) * 5.0 / 9.0,
        },
        TemperatureUnit::K => match to_unit {
            TemperatureUnit::C => input - 273.15,
            TemperatureUnit::F => input * 9.0 / 5.0 - 459.67,
            TemperatureUnit::K => input,
        },
    }
}

fn main() {
    println!("This is a temperature conversion calculator.");
    println!("It converts from/to Celsius, Farenheit, and Kelvin.");
    // The main loop keeps the program running until the user chooses to exit.
    'main: loop {
        println!("Enter a temperature(number) or E to exit:");
        let mut value_str = String::new();
        io::stdin()
            .read_line(&mut value_str)
            .expect("Failed to read user input.");
        if value_str.trim() == "E" {
            println!("Exiting the temperature conversion calculator.");
            break 'main;
        }
        let value = match parse_value(&value_str) {
            Some(v) => v,
            None => {
                println!("[ERROR] Invalid value was entered. Try again.");
                continue 'main;
            }
        };

        let from_unit: TemperatureUnit;
        // This loop re-prompts the user for a from unit until entered correctly.
        'from_unit: loop {
            println!("Enter FROM temperature unit:[Cc/Ff/Kk]");
            let mut from_unit_str = String::new();
            io::stdin()
                .read_line(&mut from_unit_str)
                .expect("Failed to read user input");
            from_unit = match parse_unit(&from_unit_str) {
                Some(u) => u,
                None => {
                    println!("[ERROR] Invalid FROM temperature unit e");
                    println!("Valid choices are:[Cc/Ff/Kk]");
                    continue 'from_unit;
                }
            };
            break 'from_unit;
        }

        let to_unit: TemperatureUnit;
        // This loop re-prompts the user for a to unit until entered correctly.
        'to_unit: loop {
            println!("Enter TO temperature unit:[Cc/Ff/Kk]");
            let mut to_unit_str = String::new();
            io::stdin()
                .read_line(&mut to_unit_str)
                .expect("Failed to read user input");
            to_unit = match parse_unit(&to_unit_str) {
                Some(u) => u,
                None => {
                    println!("[ERROR] Invalid TO temperature unit e");
                    println!("Valid choices are:[Cc/Ff/Kk]");
                    continue 'to_unit;
                }
            };
            break 'to_unit;
        }

        println!(
            "{:.2}{:?} = {:.2}{:?}",
            value,
            from_unit,
            convert(value, &from_unit, &to_unit),
            to_unit
        );
    }
}
