use std::io::stdin;

fn main() {
    let num: u32 = 20;
    let mut input = String::new();
    
    println!("Ingrese un numero sin signo");
    stdin().read_line(&mut input).expect("Failed to read user input.");
    
    let user_num: u32 = input.trim().parse().expect("Input was not a valid unsigned number");
    let result = num + user_num;

    println!("{num} + {user_num} = {}", result);
    println!("{result}^2 = {}", result * result);
}
