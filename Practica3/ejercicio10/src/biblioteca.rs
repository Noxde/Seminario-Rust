use std::collections::HashMap;
use crate::fecha::Fecha;
use chrono::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros
}

#[derive(Debug, PartialEq)]
pub enum Estado {
    Prestamo,
    Devuelto
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cliente {
    nombre: String,
    telefono: String,
    correo: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct Libro {
    isbn: u32,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Genero
}

#[derive(Debug, PartialEq)]
pub struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    vencimiento: Fecha,
    devolucion: Option<Fecha>,
    estado: Estado
}

pub struct Biblioteca {
    nombre: String,
    direccion: String,
    disposicion: HashMap<u32, u32>,
    prestamos: Vec<Prestamo>
}

impl Cliente {
    pub fn new(nombre: String, telefono: String, correo: String) -> Cliente {
        Cliente {
            nombre,
            telefono,
            correo
        }
    }
}

impl Libro {
    pub fn new(isbn: u32, titulo: String, autor: String, paginas: u32, genero: Genero) -> Libro {
        Libro {
            isbn,
            titulo,
            autor,
            paginas,
            genero
        }
    }
}

impl Prestamo {
    pub fn new(libro: Libro, cliente: Cliente, vencimiento: Fecha, estado: Estado) -> Prestamo {
        Prestamo {
            libro,
            cliente,
            vencimiento,
            devolucion: None,
            estado
        }
    }
}

impl Biblioteca {
    pub fn new(nombre: String, direccion: String, disposicion: HashMap<u32, u32>, prestamos: Vec<Prestamo>) -> Biblioteca {
        Biblioteca {
            nombre,
            direccion,
            disposicion,
            prestamos
        }
    }

    pub fn cantidad_copias(&self, isbn: u32) -> u32 {
        match self.disposicion.get(&isbn) {
            Some(value) => *value,
            None => 0
        }
    }

    pub fn decrementar_copias(&mut self, isbn: u32) {
        let libro = self.disposicion.get_mut(&isbn);
        
        match libro {
            Some(value) => {
                if *value > 0 {
                    *value -= 1;
                }
            },
            None => ()
        }
    }

    pub fn incrementar_copias(&mut self, isbn: u32) {
        let libro = self.disposicion.get_mut(&isbn);
        
        match libro {
            Some(value) => {
                *value += 1;
            },
            None => ()
        }
    }

    pub fn prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut total = 0;

        for p in self.prestamos.iter() {
            if &p.cliente == cliente && p.estado == Estado::Prestamo {
                total += 1;
            }
        }

        total
    }

    pub fn realizar_prestamo(&mut self, cliente: Cliente, libro: Libro, vencimiento: Fecha) -> bool {
        if self.prestamos_cliente(&cliente) > 5 || self.cantidad_copias(libro.isbn) == 0 {
            return false;
        }

        *self.disposicion.get_mut(&libro.isbn).unwrap() -= 1;
        self.prestamos.push(Prestamo::new(libro, cliente, vencimiento, Estado::Prestamo));
        true
    }

    pub fn prestamos_a_vencer(&self, dias: u32) -> Vec<&Prestamo> {
        let mut prestamos: Vec<&Prestamo> = Vec::new();
        let actual: DateTime<Utc> = Utc::now(); 
        let mut actual_fecha = Fecha::new(actual.day(), actual.month(), actual.year() as u32);
        actual_fecha.sumar_dias(dias);

        for p in self.prestamos.iter() {
            if actual_fecha.es_mayor(&p.vencimiento) {
                prestamos.push(p);
            }
        }

        prestamos
    }

    pub fn prestamos_vencidos(&self) -> Vec<&Prestamo> {
        let mut prestamos: Vec<&Prestamo> = Vec::new();
        let actual: DateTime<Utc> = Utc::now(); 
        let actual_fecha = Fecha::new(actual.day(), actual.month(), actual.year() as u32);

        for p in self.prestamos.iter() {
            if actual_fecha.es_mayor(&p.vencimiento) {
                prestamos.push(p);
            }
        }

        prestamos
    }

    pub fn devolver_libro(&mut self, isbn: u32, cliente: &Cliente) {
        for p in self.prestamos.iter_mut() {
            if &p.cliente == cliente && p.estado == Estado::Prestamo {
                let libro = self.disposicion.get_mut(&isbn);

                match libro {
                    Some(value) => {
                        let actual: DateTime<Utc> = Utc::now(); 
                        let actual_fecha = Fecha::new(actual.day(), actual.month(), actual.year() as u32);
                        
                        p.estado = Estado::Devuelto;
                        p.devolucion = Some(actual_fecha);
                        *value += 1;
                    },
                    None => ()
                }
            }
        }
    }
}

#[test]
fn test_cliente_new() {
    let c = Cliente::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));

    assert_eq!(String::from("Test1"), c.nombre);
    assert_eq!(String::from("Test2"), c.telefono);
    assert_eq!(String::from("Test3"), c.correo);
}

#[test]
fn test_libro_new() {
    let l = Libro::new(23671, String::from("Test1"), String::from("Test2"), 257, Genero::Tecnico);

    assert_eq!(23671, l.isbn);
    assert_eq!(String::from("Test1"), l.titulo);
    assert_eq!(String::from("Test2"), l.autor);
    assert_eq!(257, l.paginas);
    assert_eq!(Genero::Tecnico, l.genero);
}

#[test]
fn test_prestamo_new() {
    let l = Libro::new(23671, String::from("Test1"), String::from("Test2"), 257, Genero::Tecnico);
    let c = Cliente::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let f = Fecha::new(18, 6, 2024);

    let p = Prestamo::new(l, c, f, Estado::Prestamo);

    assert_eq!(Estado::Prestamo, p.estado);
    assert_eq!(23671, p.libro.isbn);
    assert_eq!(String::from("Test1"), p.cliente.nombre);
    assert_eq!(6, p.vencimiento.mes);
    assert_eq!(None, p.devolucion);
}

#[test]
fn test_biblioteca_new() {
    let b = Biblioteca::new(String::from("Test1"), String::from("Test2"), HashMap::new(), Vec::new());

    assert_eq!(String::from("Test1"), b.nombre);
    assert_eq!(String::from("Test2"), b.direccion);
    assert_eq!(true, b.disposicion.is_empty());
    assert_eq!(true, b.prestamos.is_empty());
}

#[test]
fn test_biblioteca_cantidad_copias() {
    let libros: HashMap<u32, u32> = HashMap::from([
        (275, 20),
        (134, 4),
        (394, 2),
    ]);
    let b = Biblioteca::new(String::from("Test1"), String::from("Test2"), libros, Vec::new());

    assert_eq!(20, b.cantidad_copias(275));
    assert_eq!(4, b.cantidad_copias(134));
    assert_eq!(2, b.cantidad_copias(394));
}

#[test]
fn test_biblioteca_decrementar_copias() {
    let libros: HashMap<u32, u32> = HashMap::from([
        (275, 20),
        (134, 4),
        (394, 2),
    ]);
    let mut b = Biblioteca::new(String::from("Test1"), String::from("Test2"), libros, Vec::new());
    b.decrementar_copias(275);
    assert_eq!(19, b.cantidad_copias(275));
    
    b.decrementar_copias(394);
    b.decrementar_copias(394);
    b.decrementar_copias(394);
    b.decrementar_copias(394);
    
    assert_eq!(0, b.cantidad_copias(394));
}

#[test]
fn test_biblioteca_incrementar_copias() {
    let libros: HashMap<u32, u32> = HashMap::from([
        (275, 20),
        (134, 4),
        (394, 2),
    ]);
    let mut b = Biblioteca::new(String::from("Test1"), String::from("Test2"), libros, Vec::new());
    b.incrementar_copias(275);
    assert_eq!(21, b.cantidad_copias(275));
    
    b.incrementar_copias(394);
    b.incrementar_copias(394);
    b.incrementar_copias(394);
    b.incrementar_copias(394);
    
    assert_eq!(6, b.cantidad_copias(394));
}

#[test]
fn test_biblioteca_realizar_prestamo() {
    let libros: HashMap<u32, u32> = HashMap::from([
        (275, 20),
        (134, 4),
        (394, 2),
    ]);
    let c = Cliente::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let l = Libro::new(394, String::from("Test4"), String::from("Test5"), 234, Genero::Tecnico);
   
    let mut b = Biblioteca::new(String::from("Test1"), String::from("Test2"), libros, Vec::new());
    
    b.realizar_prestamo(c, l, Fecha::new(12, 6, 2024));
    assert_eq!(1, b.cantidad_copias(394));
    assert_eq!(b.prestamos[0].estado, Estado::Prestamo);
    assert_eq!(b.prestamos[0].cliente.nombre, String::from("Test1"));
}

#[test]
fn test_prestamos_cliente() {
    let libros: HashMap<u32, u32> = HashMap::from([
        (275, 20),
        (134, 4),
        (394, 2),
    ]);
    let c = Cliente::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let l = Libro::new(394, String::from("Test4"), String::from("Test5"), 234, Genero::Tecnico);
    let l2 = Libro::new(275, String::from("Test6"), String::from("Test7"), 120, Genero::Otros);
   
    let mut b = Biblioteca::new(String::from("Test1"), String::from("Test2"), libros, Vec::new());
    
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(12, 6, 2024));
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(12, 6, 2024));
    b.realizar_prestamo(c.clone(), l, Fecha::new(12, 6, 2024)); // No agrega, no hay mas copias

    assert_eq!(2, b.prestamos_cliente(&c));

    b.realizar_prestamo(c.clone(), l2.clone(), Fecha::new(12, 6, 2024));
    b.realizar_prestamo(c.clone(), l2.clone(), Fecha::new(12, 6, 2024));
    b.realizar_prestamo(c.clone(), l2.clone(), Fecha::new(12, 6, 2024));
    b.realizar_prestamo(c.clone(), l2.clone(), Fecha::new(12, 6, 2024));
    b.realizar_prestamo(c.clone(), l2.clone(), Fecha::new(12, 6, 2024));
    b.realizar_prestamo(c, l2, Fecha::new(12, 6, 2024));
    
    assert_eq!(6, b.prestamos_cliente(&Cliente::new(String::from("Test1"), String::from("Test2"), String::from("Test3"))));
    
}

#[test]
fn test_biblioteca_prestamos_a_vencer() {
    let libros: HashMap<u32, u32> = HashMap::from([
        (275, 20),
        (134, 4),
        (394, 2),
    ]);
    let c = Cliente::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let l = Libro::new(275, String::from("Test6"), String::from("Test7"), 120, Genero::Otros);
    let mut b = Biblioteca::new(String::from("Test1"), String::from("Test2"), libros, Vec::new());

    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(17, 5, 2024));
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(20, 6, 2024));
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(15, 5, 2024));
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(18, 5, 2024));

    assert_eq!(3, b.prestamos_a_vencer(7).len());
}

#[test]
fn test_biblioteca_prestamos_vencidos() {
    let libros: HashMap<u32, u32> = HashMap::from([
        (275, 20),
        (134, 4),
        (394, 2),
    ]);
    let c = Cliente::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let l = Libro::new(275, String::from("Test6"), String::from("Test7"), 120, Genero::Otros);
    let mut b = Biblioteca::new(String::from("Test1"), String::from("Test2"), libros, Vec::new());

    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(5, 4, 2024));
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(2, 4, 2024));
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(14, 5, 2024));
    b.realizar_prestamo(c, l, Fecha::new(10, 5, 2024));

    assert_eq!(3, b.prestamos_vencidos().len());
}

fn test_biblioteca_devolver_libro() {
    let libros: HashMap<u32, u32> = HashMap::from([
        (275, 20),
        (134, 4),
        (394, 2),
    ]);
    let c = Cliente::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let l = Libro::new(275, String::from("Test6"), String::from("Test7"), 120, Genero::Otros);
    let mut b = Biblioteca::new(String::from("Test1"), String::from("Test2"), libros, Vec::new());

    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(5, 4, 2024));
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(2, 4, 2024));
    b.realizar_prestamo(c.clone(), l.clone(), Fecha::new(14, 5, 2024));
    b.realizar_prestamo(c.clone(), l, Fecha::new(10, 5, 2024));

    assert_eq!(16, b.cantidad_copias(275));
    b.devolver_libro(275, &c);
    assert_eq!(20, b.cantidad_copias(275));
    assert_eq!(0, b.prestamos_cliente(&c));
}