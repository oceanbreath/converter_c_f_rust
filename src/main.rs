use std::io;

const C: f32 = 32.0;
const D: f32 = 9.0 / 5.0;

fn c_to_f(temperature: &f32) -> f32 {
    let result = temperature * D + C;
    println!("The result of convertion from C to F is: {}", result);
    result
}

fn f_to_c(temperature: &f32) -> f32 {
    let result = (temperature - C) / D;
    println!("The result of convertion from F to C is: {}", result);
    result
}

fn input_temperature() -> f32 {
    loop {
        println!("Input temperature:");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).unwrap();
        match temp.trim().parse::<f32>() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    }
}

fn main() {
    println!("Temperature convert");
    println!("1 - from Celsius to Fahrenheit");
    println!("2 - from Fahrenheit to Celsius");
    println!("any other key - exit");

    loop {
        println!("Input your choice:");
        let mut choice_input = String::new();
        io::stdin().read_line(&mut choice_input).unwrap();
        //let temperature: f32 = input_temperature();
        let result = match choice_input.trim() {
            "1" => c_to_f(&input_temperature()),
            "2" => f_to_c(&input_temperature()),
            _ => {
                println!("Exiting...");
                break;
            }
        };

        //let result = convert(temperature, choice);
        println!("{}", result);
    }
}
