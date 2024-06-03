// 1
pub fn es_par(n: i32) -> bool {
    n % 2 == 0
}
#[test]
fn test_es_par() {
    assert_eq!(true, es_par(4));
    assert_ne!(true, es_par(5));
}
// 2
pub fn es_primo(n: i32) -> bool {
    for d in 2..n {
        if n % d == 0 {
            return false;
        } 
    }
    true
}
#[test]
fn test_es_primo() {
    assert_eq!(true, es_primo(7));
    assert_ne!(true, es_primo(4));
}
// 3
pub fn suma_pares(arr: [i32;5]) -> i32 {
    let mut sum = 0;
    for x in arr.iter() {
        if x % 2 == 0 {
            sum += *x;
        }
    }
    sum
}
#[test]
fn test_suma_pares() {
    assert_eq!(16, suma_pares([1,2,3,6,8]));
}
// 4
pub fn cantidad_impares(arr: [i32;5]) -> i32 {
    let mut cant = 0;
    for x in arr.iter() {
        if !es_par(*x) {
            cant += 1;
        }
    }
    cant
}
#[test]
fn test_cantidad_impares() {
    assert_eq!(3, cantidad_impares([1,2,3,4,5]));
}
// 5
pub fn duplicar_valores(arr: [f64;5]) -> [f64;5] {
    let mut nuevo = arr;
    for i in nuevo.iter_mut() {
        *i = *i*2.0;
    }
    nuevo
}
#[test]
fn test_duplicar_valores() {
    assert_eq!([2.0,4.0,6.0,8.0,10.0], duplicar_valores([1.0,2.0,3.0,4.0,5.0]));
}
// 6
pub fn longitud_de_cadenas(arr: [String;5]) -> [i32;5] {
    let mut retorno: [i32; 5] = [0;5];
    for (i, s) in arr.iter().enumerate() {
        retorno[i] = s.len() as i32;
    }
    retorno    
}
#[test]
fn test_longitud_de_cadenas() {
    let strings = [String::from("123"), String::from("1234"), String::from("12345"), String::from("123456"), String::from("1234567")];

    assert_eq!([3,4,5,6,7], longitud_de_cadenas(strings));
}
// 7
pub fn cantidad_de_mayores(arr: [i32;5], limite: i32) -> i32 {
    let mut cant = 0;
    
    for i in arr.iter() {
        if *i > limite {
            cant += 1;
        }
    } 
    cant
}
#[test]
fn test_cantidad_de_mayores() {
    assert_eq!(3, cantidad_de_mayores([5,2,1,10,15], 2));
}
// 8
pub fn sumar_arreglos(arr1: [f64;5], arr2: [f64;5]) -> [f64;5] {
    let mut suma: [f64;5] = [0.0;5];

    for i in 0..5
 {
    suma[i] = arr1[i] + arr2[i];
 }
    suma
}
#[test]
fn test_sumar_arreglos() {
    let arr1 = [1.0, 2.0, 3.0, 4.0, 5.0];
    let arr2 = [6.0, 7.0, 8.0, 9.0, 10.0];
    assert_eq!([7.0, 9.0, 11.0, 13.0, 15.0], sumar_arreglos(arr1, arr2));
}
// 9
pub fn cantidad_en_rango(arr: [i32;5], inf: i32, sup: i32) -> i32 {
    let mut cant = 0;

    for i in arr.iter() {
        if inf <= *i && *i <= sup {
            cant += 1;
        }
    }

    cant
}
#[test]
fn test_cantidad_en_rango() {
    let arr = [1,2,3,4,5];
    assert_eq!(3, cantidad_en_rango(arr, 3, 5));
}
// 10