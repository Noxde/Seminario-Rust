pub struct _Triangulo(f64, f64, f64);

#[derive(PartialEq, Debug)]
pub enum _Tipo {
    Equilatero,
    Isosceles,
    Escaleno
}

impl _Triangulo {
    pub fn _new(a: f64, b: f64, c: f64) -> _Triangulo {
        _Triangulo(a, b, c)
    }

    pub fn _determinar_tipo(&self) -> _Tipo {
        if self.0 == self.1 && self.0 == self.2 && self.1 == self.2 {
            _Tipo::Equilatero
        } else if self.0 == self.1 || self.0 == self.2 || self.1 == self.2 {
            _Tipo::Isosceles
        } else {
            _Tipo::Escaleno
        }
    }

    pub fn _calcular_area(&self) -> f64 {
        let s = (self.0 + self.1 + self.2) / 2.0;
        f64::sqrt(s * (s - self.0) * (s - self.1) * (s - self.2))
    }

    pub fn _calcular_perimetro(&self) -> f64 {
        self.0 + self.1 + self.2
    }
}

#[test]
fn test_new() {
    let triangulo = _Triangulo::_new(3.0, 4.0, 5.0);
    assert_eq!(triangulo.0, 3.0);
    assert_eq!(triangulo.1, 4.0);
    assert_eq!(triangulo.2, 5.0);
}
#[test]
fn test_determinar_tipo() {
    let equilatero = _Triangulo::_new(2.0, 2.0, 2.0);
    let isosceles = _Triangulo::_new(2.0, 3.0, 2.0);
    let escaleno = _Triangulo::_new(2.0, 3.0, 4.0);

    assert_eq!(_Tipo::Equilatero, equilatero._determinar_tipo());
    assert_eq!(_Tipo::Isosceles, isosceles._determinar_tipo());
    assert_eq!(_Tipo::Escaleno, escaleno._determinar_tipo());
}
#[test]
fn test_calcular_area() {
    let t = _Triangulo(2.0, 5.0, 4.0);
    let tr = _Triangulo(2.5, 3.4, 1.8);

    assert_eq!(3.799671038392666, t._calcular_area());
    assert_eq!(2.189678914818335, tr._calcular_area());
}
#[test]
fn test_calcular_perimetro() {
    let t = _Triangulo(2.0, 5.0, 4.0);
    
    assert_eq!(11.0, t._calcular_perimetro());
}