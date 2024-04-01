fn main() {
    const VALOR: i32 = 5;
    let mut array = [1,2,3,4,5,6];
    array.iter_mut().for_each(|n| *n = *n * VALOR);
}
