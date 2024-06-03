trait Primo {
    fn es_primo(&self) -> bool;
}

impl Primo for i32 {
    fn es_primo(&self) -> bool {
        let mut n = 0;
        
        if *self == 1{
            return false;
        }
        
        if *self == 2 {
            return true
        }

        for i in 2..=*self {
            if *self % i == 0 {
                println!("{i}");
                n += 1;
            }
            println!("{n}");
            if n > 2 {
                return false
            }
        }

        true
    }
}

#[test]
fn test_es_primo() {
    let primo = 7;
    let no_primo = 6;

    assert_eq!(true, primo.es_primo());
    assert_eq!(false, no_primo.es_primo());
}

fn main() {
    let numeros = vec![1,2,3,4,5,6,7,8,9,10,11,12];

    let count = numeros.iter().filter(|x| x.es_primo()).count();
    println!("Hay {count} numeros primos");
}
