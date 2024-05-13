pub struct Rectangulo {
    pub longitud: i32,
    pub ancho: i32   
}

impl Rectangulo {
    pub fn new(longitud: i32, ancho: i32) -> Rectangulo {
        Rectangulo {
            longitud,
            ancho
        }
    }

    pub fn calcular_area(&self) -> i32 {
        self.longitud * self.ancho
    }

    pub fn calcular_perimetro(&self) -> i32 {
        2 * (self.longitud + self.ancho)
    }

    pub fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[test]
fn test_new() {
    let rectangulo = Rectangulo {
        longitud: 20,
        ancho: 32
    };

    assert_eq!(20, rectangulo.longitud);
    assert_eq!(32, rectangulo.ancho);
}

#[test]
fn test_calcular_area() {
    let rectangulo = Rectangulo {
        longitud: 22,
        ancho: 35
    };

    assert_eq!(770, rectangulo.calcular_area());
}

#[test]
fn test_calcular_perimetro() {
    let rectangulo = Rectangulo {
        longitud: 22,
        ancho: 35
    };

    assert_eq!(114, rectangulo.calcular_perimetro());
}

#[test]
fn test_es_cuadrado() {
    let cuadrado = Rectangulo {
        longitud: 10,
        ancho: 10
    };

    let rectangulo = Rectangulo {
        longitud: 22,
        ancho: 35
    };

    assert_eq!(true, cuadrado.es_cuadrado());
    assert_ne!(true, rectangulo.es_cuadrado());
}