use std::io::stdin;

fn main() {
    let valor = 3.14;
    let mut input = String::new();
    
    println!("Ingrese un numero:");
    stdin().read_line(&mut input).expect("Failed to read user input.");

    let user_value: f64 = input.trim().parse().expect("Input was not a valid number.");

    println!("{valor} + {user_value} = {}", valor + user_value);
    println!("{valor} - {user_value} = {}", valor - user_value);
    println!("{valor} * {user_value} = {}", valor * user_value);
    println!("{valor} / {user_value} = {}", valor / user_value);
}
