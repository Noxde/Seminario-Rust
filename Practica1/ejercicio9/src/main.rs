fn main() {
    let numeros = [1,2,3,4,5];
    let suma: i32 = numeros.iter().sum();

    println!("La suma de {:?} es {suma}", numeros);
}
