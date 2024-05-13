pub struct Producto {
    nombre: String,
    precio: f64,
    id: u32
}

impl Producto {
    pub fn new(nombre: String, precio: f64, id: u32) -> Producto {
        Producto {
            nombre,
            precio,
            id
        }
    }

    pub fn calcular_impuestos(&self, porcentaje: f64) -> f64 {
        self.precio * (porcentaje/100.0)
    }

    pub fn aplicar_descuento(&self, porcentaje: f64) -> f64 {
        self.precio * (porcentaje/100.0) * -1.0
    }

    pub fn calcular_precio_total(&self, impuestos: f64, descuento: f64) -> f64 {
        (self.precio + self.calcular_impuestos(impuestos)) + self.aplicar_descuento(descuento)
    }
}

#[test]
fn test_new() {
    let p = Producto::new("Test".to_string(), 500.0, 23);

    assert_eq!(String::from("Test"), p.nombre);
    assert_eq!(23, p.id);
    assert_eq!(500.0, p.precio);
}
#[test]
fn test_calcular_impuestos() {
    let p = Producto::new("Test".to_string(), 500.0, 23);

    assert_eq!(250.0, p.calcular_impuestos(50.0));
}
#[test]
fn test_aplicar_descuento() {
    let p = Producto::new("Test".to_string(), 500.0, 23);

    assert_eq!(250.0, p.aplicar_descuento(-50.0));
}
#[test]
fn test_calcular_precio_total() {
    let p = Producto::new("Test".to_string(), 500.0, 23);

    assert_eq!(625.0, p.calcular_precio_total(50.0, 25.0));
}
