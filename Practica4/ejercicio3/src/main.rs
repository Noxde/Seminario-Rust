mod streaming_rust;
mod fecha;

fn main() {
    let mut t = fecha::Fecha::new(1, 4, 2024);
    t.sumar_dias(30);

    println!("{:?}", t);
    println!("{:?}", t.es_mayor(&fecha::Fecha::new(2, 6, 2024)));
}
