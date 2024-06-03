use crate::fecha::Fecha;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Categoria {
    Alimentos,
    Bebidas,
    Electrodomesticos,
    Ropa,
}

#[derive(Clone)]
struct Producto {
    nombre: String,
    categoria: Categoria,
    precio_base: f64
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct InfoPersona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
}

#[derive(Clone)]
pub struct Cliente {
    info: InfoPersona,
    news_letter: Option<String>
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Vendedor {
    info: InfoPersona,
    legajo: String,
    antiguedad: u8,
    salario: u32
}

#[derive(Clone)]
enum MedioDePago {
    TarjetaDeCredito,
    TarjetaDeDebito,
    TransferenciaBancaria,
    Efectivo
}

#[derive(Clone)]
struct Venta {
    nro: u32,
    fecha: Fecha,
    cliente: Cliente,
    vendedor: Vendedor,
    pago: MedioDePago,
    productos: Vec<Producto>
}

pub struct Sistema {
    ventas: Vec<Venta>
}

impl Sistema {
    pub fn new() -> Sistema {
        Sistema {
            ventas: Vec::new()
        }
    }

    pub fn crear_venta(&mut self, nro: u32, fecha: Fecha, cliente: Cliente, vendedor: Vendedor, pago: MedioDePago, productos: Vec<Producto>) {
        self.ventas.push(Venta::new(nro, fecha, cliente, vendedor, pago, productos));
    }

    fn obtener_descuento(&self, producto: &Producto) -> f64 {
        match producto.categoria {
            Categoria::Alimentos => producto.precio_base * 0.2,
            Categoria::Bebidas => producto.precio_base * 0.1,
            Categoria::Electrodomesticos => producto.precio_base * 0.3,
            Categoria::Ropa => producto.precio_base * 0.15
        }
    }

    pub fn calcular_precio_final(&self, numero: u32) -> Option<f64> {
        let venta = self.ventas.iter().find(|v| v.nro == numero);
        
        if let Some(v) = venta {
            let mut total: f64 = 0.0;
            for p in &v.productos {
                total += p.precio_base - self.obtener_descuento(&p);
            }

            if v.cliente.news_letter.is_some() {
                total -= total * 0.1;
            }

            Some(total)
        } else {
            None
        }
    }

    pub fn reporte_ventas(&self) -> Option<(HashMap<Categoria, u32>, HashMap<Vendedor, u32>)> {
        if self.ventas.is_empty() {
            return None;
        }
        
        let mut cat_map: HashMap<Categoria, u32> = HashMap::new();
        let mut vend_map: HashMap<Vendedor, u32> = HashMap::new();
    
        for v in &self.ventas {
            *vend_map.entry(v.vendedor.clone()).or_insert(0) += 1;
            for p in &v.productos {
                *cat_map.entry(p.categoria.clone()).or_insert(0) += 1;
            }
        }

        Some((cat_map, vend_map))
    }
}

impl Venta {
    pub fn new(nro: u32, fecha: Fecha, cliente: Cliente, vendedor: Vendedor, pago: MedioDePago, productos: Vec<Producto>) -> Venta {
        Venta {
            nro,
            fecha,
            cliente,
            vendedor,
            pago,
            productos
        }
    }
}

impl Producto {
    pub fn new(nombre: String, categoria: Categoria, precio_base: f64) -> Producto {
        Producto {
            nombre,
            categoria,
            precio_base,
        }
    }
}

impl InfoPersona {
    pub fn new(nombre: String, apellido: String, direccion: String, dni: String) -> InfoPersona {
        InfoPersona {
            nombre,
            apellido,
            direccion,
            dni
        }
    }
}

impl Cliente {
    pub fn new(info: InfoPersona, news_letter: Option<String>) -> Cliente {
        Cliente {
            info,
            news_letter
        }
    }
}

impl Vendedor {
    pub fn new(info: InfoPersona, legajo: String, antiguedad: u8, salario: u32) -> Vendedor {
        Vendedor {
            info,
            legajo,
            antiguedad,
            salario  
        }
    }
}

#[test]
fn test_cliente_new() {
    let info = InfoPersona::new(String::from("John"), String::from("Doe"), String::from("123 Main St"), String::from("12345678"));
    let cliente = Cliente::new(info.clone(), Some(String::from("john.doe@example.com")));
    assert_eq!(cliente.info, info);
    assert_eq!(cliente.news_letter, Some(String::from("john.doe@example.com")));
}

#[test]
fn test_vendedor_new() {
    let info = InfoPersona::new(String::from("John"), String::from("Doe"), String::from("123 Main St"), String::from("12345678"));
    let vendedor = Vendedor::new(info.clone(), String::from("1234"), 5, 50000);
    assert_eq!(vendedor.info, info);
    assert_eq!(vendedor.legajo, "1234");
    assert_eq!(vendedor.antiguedad, 5);
    assert_eq!(vendedor.salario, 50000);
}

#[test]
fn test_info_persona_new() {
    let info = InfoPersona::new(String::from("John"), String::from("Doe"), String::from("123 Main St"), String::from("12345678"));
    assert_eq!(info.nombre, "John");
    assert_eq!(info.apellido, "Doe");
    assert_eq!(info.direccion, "123 Main St");
    assert_eq!(info.dni, "12345678");
}

#[test]
fn test_sistema_new() {
    let sistema = Sistema::new();
    assert!(sistema.ventas.is_empty());
}

#[test]
fn test_sistema_crear_venta() {
    let mut sistema = Sistema::new();
    let fecha = Fecha::new(20, 12, 2022);
    let info = InfoPersona::new(String::from("John"), String::from("Doe"), String::from("123 Main St"), String::from("12345678"));
    let cliente = Cliente::new(info.clone(), Some(String::from("john.doe@example.com")));
    let vendedor = Vendedor::new(info.clone(), String::from("1234"), 5, 50000);
    let producto = Producto::new(String::from("Producto"), Categoria::Alimentos, 100.0);
    let productos = vec![producto];

    sistema.crear_venta(1, fecha, cliente, vendedor, MedioDePago::Efectivo, productos);

    assert_eq!(sistema.ventas.len(), 1);
    assert_eq!(sistema.ventas[0].vendedor.info.nombre, String::from("John"));
}

#[test]
fn test_sistema_obtener_descuento() {
    let sistema = Sistema::new();
    let producto_alimentos = Producto::new(String::from("Producto Alimentos"), Categoria::Alimentos, 100.0);
    let producto_bebidas = Producto::new(String::from("Producto Bebidas"), Categoria::Bebidas, 100.0);
    let producto_electrodomesticos = Producto::new(String::from("Producto Electrodomesticos"), Categoria::Electrodomesticos, 100.0);
    let producto_ropa = Producto::new(String::from("Producto Ropa"), Categoria::Ropa, 100.0);

    let descuento_alimentos = sistema.obtener_descuento(&producto_alimentos);
    let descuento_bebidas = sistema.obtener_descuento(&producto_bebidas);
    let descuento_electrodomesticos = sistema.obtener_descuento(&producto_electrodomesticos);
    let descuento_ropa = sistema.obtener_descuento(&producto_ropa);

    assert_eq!(descuento_alimentos, 20.0);
    assert_eq!(descuento_bebidas, 10.0);
    assert_eq!(descuento_electrodomesticos, 30.0);
    assert_eq!(descuento_ropa, 15.0);
}

#[test]
fn test_sistema_calcular_precio_final() {
    let mut sistema = Sistema::new();
    let fecha = Fecha::new(23, 12, 2022);
    let info = InfoPersona::new(String::from("John"), String::from("Doe"), String::from("123 Main St"), String::from("12345678"));
    let cliente = Cliente::new(info.clone(), Some(String::from("john.doe@example.com")));
    let vendedor = Vendedor::new(info.clone(), String::from("1234"), 5, 50000);
    let producto = Producto { nombre: String::from("Producto"), categoria: Categoria::Alimentos, precio_base: 100.0 };
    let productos = vec![producto];

    sistema.crear_venta(1, fecha, cliente, vendedor, MedioDePago::Efectivo, productos);

    let precio_final = sistema.calcular_precio_final(1);

    assert_eq!(precio_final, Some(72.0)); // 100 - 20% descuento - 10% news_letter
}

#[test]
fn test_sistema_reporte_ventas() {
    let mut sistema = Sistema::new();
    let fecha = Fecha::new(20, 12, 2022);
    
    let info = InfoPersona::new(String::from("John"), String::from("Doe"), String::from("123 Main St"), String::from("12345678"));
    let info1 = InfoPersona::new(String::from("Foo"), String::from("Bar"), String::from("123 Main St"), String::from("12345678"));
    
    let cliente = Cliente::new(info.clone(), Some(String::from("john.doe@example.com")));
    let vendedor = Vendedor::new(info, String::from("1234"), 5, 50000);
    let vendedor1 = Vendedor::new(info1, String::from("1234"), 5, 50000);
    
    let productos = vec![
        Producto::new(String::from("Producto"), Categoria::Alimentos, 100.0),
        Producto::new(String::from("Producto1"), Categoria::Electrodomesticos, 500.0),
        Producto::new(String::from("Producto2"), Categoria::Bebidas, 50.0)
    ];
    let productos1 = vec![
        Producto::new(String::from("Producto3"), Categoria::Electrodomesticos, 700.0),
        Producto::new(String::from("Producto4"), Categoria::Electrodomesticos, 500.0),
        Producto::new(String::from("Producto5"), Categoria::Ropa, 100.0)
    ];

    sistema.crear_venta(1, fecha.clone(), cliente.clone(), vendedor.clone(), MedioDePago::Efectivo, productos);
    sistema.crear_venta(2, fecha, cliente, vendedor1.clone(), MedioDePago::TransferenciaBancaria, productos1);

    if let Some((cat_map, vend_map)) = sistema.reporte_ventas() {
        assert_eq!(cat_map.get(&Categoria::Alimentos), Some(&1));
        assert_eq!(cat_map.get(&Categoria::Electrodomesticos), Some(&3));
        assert_eq!(cat_map.get(&Categoria::Bebidas), Some(&1));
        assert_eq!(cat_map.get(&Categoria::Ropa), Some(&1));
        
        
        assert_eq!(vend_map.get(&vendedor), Some(&1));
        assert_eq!(vend_map.get(&vendedor1), Some(&1));
    }

    let sistema = Sistema::new();
    assert_eq!(None, sistema.reporte_ventas());

}
