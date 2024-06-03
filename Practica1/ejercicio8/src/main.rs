use std::io::stdin;

fn main() {
    const CADENA: &str = "Esta es una cadena de prueba";
    let mut input = String::new();
    
    println!("Ingrese un caracter a buscar:");
    stdin().read_line(&mut input).expect("Failed to read user input.");
    
    let search: char = input.trim().parse().expect("Couldnt parse to char.");

    let count = CADENA.matches(search).count(); 

    println!("{search} aparece {count} veces en {CADENA}"); 
}
