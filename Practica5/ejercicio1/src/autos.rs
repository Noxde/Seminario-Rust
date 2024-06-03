use std::{fmt::Display, fs::File, io::Write};

use serde::{de::value::Error, Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct ErrorMax(u32);

impl Display for ErrorMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No se puedo ingresar otro auto. La capacidad maxima es {}", self.0)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Rojo,
    Azul,
    Amarillo,
    Verde,
    Blanco,
    Negro
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Auto {
    marca: String,
    modelo: String,
    anio: u32,
    precio: f64,
    color: Color
}

pub struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad: u32,
    pub autos: Vec<Auto>
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad: u32) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad,
            autos: Vec::new()
        }
    }

    fn escribir_autos(&self) {
        match File::create("./autos.json") {
            Ok(mut file) => {
                let b_s = serde_json::to_string_pretty(&self.autos).unwrap();
                file.write_all(&b_s.as_bytes()).expect("Error al escribir el archivo autos.json");
            },
            Err(error) => {
                println!("Error al crear archivo: {error}");
            }
        };
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> Result<(), ErrorMax> {
        if self.capacidad > self.autos.len() as u32 + 1 {
            self.autos.push(auto);
            self.escribir_autos();
            Ok(())
        } else {
            Err(ErrorMax(self.capacidad))?
        }
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) {
        let mut indice = None;
        for (i, a) in self.autos.iter().enumerate() {
            if auto == a {
                indice = Some(i);
            }
        }
        if indice.is_some() {
            self.autos.swap_remove(indice.unwrap());
            self.escribir_autos();
        }
    }

    // Retorna una referencia que va a ser valida mientras self.autos exista
    pub fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        for a in self.autos.iter() {
            if a == auto {
                return Some(a);
            } 
        }
        None
    }
}

impl Auto {
    pub fn new(marca: String, modelo: String, anio: u32, precio: f64, color: Color) -> Auto {
        Auto {
            marca, 
            modelo,
            anio,
            precio,
            color,
        }
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut precio_total = self.precio;
    
        match self.color {
            Color::Rojo | Color::Amarillo | Color::Azul => {
                precio_total *= 1.25;
            },
            _ => {
                precio_total -= precio_total * 0.1;
            }
        }

        if self.marca == String::from("BMW") {
            precio_total *= 1.5;
        }
        if self.anio < 2000 {
            precio_total -= precio_total * 0.05; 
        }

        precio_total
    }
}

#[test]
fn test_concesionario_new() {
    let mut c = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 50);
 
    assert_eq!(String::from("test1"), c.nombre);
    assert_eq!(String::from("test calle"), c.direccion);
    assert_eq!(50, c.capacidad);
}

#[test]
fn test_agregar_auto() {
    let mut c = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 50);
   
    c.agregar_auto(Auto::new(String::from("Test1"), String::from("Test2"), 2003, 4500.0, Color::Rojo));
    c.agregar_auto(Auto::new(String::from("Test3"), String::from("Test4"), 2007, 3500.0, Color::Amarillo));
    c.agregar_auto(Auto::new(String::from("Test5"), String::from("Test6"), 1998, 6500.0, Color::Blanco));

    assert_eq!(3, c.autos.len());

    let mut c2 = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 2);
    c2.agregar_auto(Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde));
    c2.agregar_auto(Auto::new(String::from("Test3"), String::from("Test4"), 2020, 5250.0, Color::Azul));
    
    let res = c2.agregar_auto(Auto::new(String::from("Test5"), String::from("Test6"), 2021, 10000.0, Color::Rojo));

    assert_eq!(ErrorMax(2), res.err().unwrap());
}

#[test]
fn test_eliminar_auto() {
    let mut c = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 5);
    c.agregar_auto(Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde));
    c.agregar_auto(Auto::new(String::from("Test3"), String::from("Test4"), 2020, 5250.0, Color::Azul));
    c.agregar_auto(Auto::new(String::from("Test5"), String::from("Test6"), 2021, 10000.0, Color::Rojo));

    let a = Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde);

    c.eliminar_auto(&a);
    assert_eq!(2, c.autos.len());
}

#[test]
fn test_buscar_auto() {
    let mut c = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 5);
    c.agregar_auto(Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde));
    c.agregar_auto(Auto::new(String::from("Test3"), String::from("Test4"), 2020, 5250.0, Color::Azul));
    c.agregar_auto(Auto::new(String::from("Test5"), String::from("Test6"), 2021, 10000.0, Color::Rojo));
    
    let a = Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde);
    let n = Auto::new(String::from("Test23"), String::from("Test2"), 2000, 2500.0, Color::Verde);


    assert_eq!(Some(&a), c.buscar_auto(&a));
    assert_eq!(None, c.buscar_auto(&n));
}

#[test]
fn test_auto_new() {
    let a = Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde);

    assert_eq!(String::from("Test1"), a.marca);
    assert_eq!(String::from("Test2"), a.modelo);
    assert_eq!(2000, a.anio);
    assert_eq!(2500.0, a.precio);
    assert_eq!(Color::Verde, a.color);
}

#[test]
fn test_auto_calcular_precio() {
    let a = Auto::new(String::from("BMW"), String::from("Test1"), 2005, 2500.0, Color::Verde);
    let a2 = Auto::new(String::from("Test1"), String::from("Test2"), 1998, 3400.0, Color::Azul);

    assert_eq!(3375.0, a.calcular_precio());
    assert_eq!(4037.5, a2.calcular_precio());
 }
