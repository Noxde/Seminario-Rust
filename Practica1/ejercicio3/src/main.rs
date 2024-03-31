use std::io::stdin;

fn main() {
    let mut bool = true;
    let mut input = String::new();

    println!("Ingrese un valor booleano (true/false)");
    stdin().read_line(&mut input).unwrap();

    let user_bool: bool = input.trim().parse().expect("El valor ingresado no es un booleano valido.");
    print!("{bool} and {user_bool}");
    bool = user_bool && bool;
    println!(" = {bool}");

    print!("{bool} or {user_bool}");
    bool = user_bool || bool;
    println!(" = {bool}");
}
