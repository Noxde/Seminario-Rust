mod xyz;
mod fecha;
use chrono::prelude::*;
use rand::random;

fn main() {
    println!("{}", format!("{}#{}", "BTC", rand::random::<u32>()));
}
