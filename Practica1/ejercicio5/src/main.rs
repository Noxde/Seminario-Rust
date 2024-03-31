use std::io::stdin;

fn main() {
    let cadena = String::from("Cadena");
    let mut input = String::new();

    println!("Ingrese una cadena:");
    stdin().read_line(&mut input).expect("Failed to read user input.");

    let cadena_nueva = format!("{cadena} {}", input.trim());
    println!("{}", cadena_nueva.to_uppercase());

}
