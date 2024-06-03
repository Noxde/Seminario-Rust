use std::io::stdin;

fn main() {
    let array = ["Test", "Test1", "Test2", "Test3", "Test4"];

    let mut input = String::new();
    
    println!("Ingrese una cadena a buscar");
    stdin().read_line(&mut input).expect("Failed to read user input.");
    
    let search = input.trim();
    
}
