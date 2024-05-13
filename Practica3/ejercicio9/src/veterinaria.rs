use std::collections::VecDeque;
use crate::fecha::Fecha;

#[derive(Debug, PartialEq, Clone)]
pub struct Duenio {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mascota {
    nombre: String,
    edad: u8,
    tipo: Animal,
    duenio: Duenio
}


#[derive(Debug, PartialEq, Clone)]
pub struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    visita: Option<Fecha>
}


pub struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola: VecDeque<Mascota>,
    atenciones_realizadas: Vec<Atencion>
}

pub struct Cadena {
    veterinarias: Vec<Veterinaria>
}

impl Duenio {
    pub fn new(nombre: String, direccion: String, telefono: String) -> Duenio {
        Duenio {
            nombre,
            direccion,
            telefono
        }
    }
}

impl Mascota {
    pub fn new(nombre: String, edad: u8, tipo: Animal, duenio: Duenio) -> Mascota {
        Mascota {
            nombre,
            edad,
            tipo,
            duenio
        }
    }
}

impl Atencion {
    pub fn new(mascota: Mascota, diagnostico: String, tratamiento: String, visita: Option<Fecha>) -> Atencion {
        Atencion {
            mascota,
            diagnostico,
            tratamiento,
            visita
        }
    }
}

impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u32) -> Veterinaria {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola: VecDeque::new(),
            atenciones_realizadas: Vec::new()
        }
    }

    pub fn agregar_cola(&mut self, mascota: Mascota) {
        self.cola.push_back(mascota);
    }

    pub fn agregar_cola_prioridad(&mut self, mascota: Mascota) {
        self.cola.push_front(mascota);
    }

    pub fn atender_prox(&mut self) -> Option<Mascota> {
        self.cola.pop_front()
    }

    pub fn eliminar(&mut self, mascota: &Mascota) -> bool {
        let mut index = None;

        for (i, m) in self.cola.iter().enumerate() {
            if m == mascota {
                index = Some(i)
            }
        }

        if index.is_some() {
            self.cola.remove(index.unwrap());
            true
        } else {
            false
        }
    }

    pub fn registrar_atencion(&mut self, mascota: Mascota, diagnostico: String, tratamiento: String, prox_visita: Option<Fecha>) {
        self.atenciones_realizadas.push(Atencion::new(mascota, diagnostico, tratamiento, prox_visita));
    }

    pub fn buscar_atencion(&self, nombre_mascota: &str, nombre_duenio: &str, telefono_duenio: &str) -> Option<&Atencion> {
        for m in self.atenciones_realizadas.iter() {
            if m.mascota.nombre == nombre_mascota &&
                m.mascota.duenio.nombre == nombre_duenio &&
                m.mascota.duenio.telefono == telefono_duenio {
                    return Some(m);
                }
        }
        None
    }

    pub fn modificar_diagnostico(&mut self, atencion: &Atencion, nuevo_diagnostico: &String) {
        for m in self.atenciones_realizadas.iter_mut() {
            if m == atencion {
                m.diagnostico = nuevo_diagnostico.clone();
            }
        }
    }

    pub fn modificar_fecha(&mut self, atencion: &Atencion, nueva_fecha: Option<Fecha>) {
        for m in self.atenciones_realizadas.iter_mut() {
            if m == atencion {
                m.visita = nueva_fecha.clone();
            }
        }
    }

    pub fn eliminar_atencion(&mut self, atencion: &Atencion) {
        let mut index = None;

        for (i, m) in self.atenciones_realizadas.iter().enumerate() {
            if m == atencion {
                index = Some(i);
            }
        }

        if index.is_some() {
            self.atenciones_realizadas.remove(index.unwrap());
        }
    }
}

impl Cadena {
    pub fn new() -> Cadena {
        Cadena {
            veterinarias: Vec::new()
        }
    }
}

#[test]
fn test_duenio_new() {
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));

    assert_eq!(String::from("Test1"), d.nombre);
    assert_eq!(String::from("Test2"), d.direccion);
    assert_eq!(String::from("Test3"), d.telefono);
}

#[test]
fn test_mascota_new() {
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);

    assert_eq!(String::from("Test4"), m.nombre);
    assert_eq!(4, m.edad);
    assert_eq!(Animal::Perro, m.tipo);
    assert_eq!(String::from("Test1"), m.duenio.nombre);
}

#[test]
fn test_atencion_new() {
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);
    
    let a = Atencion::new(m, String::from("Test5"), String::from("Test6"), None);
    assert_eq!(String::from("Test4"), a.mascota.nombre);
    assert_eq!(String::from("Test5"), a.diagnostico);
    assert_eq!(String::from("Test6"), a.tratamiento);

}

#[test]
fn test_veterinaria_new() {
    let v = Veterinaria::new(String::from("Test1"), String::from("Test2"), 5);

    assert_eq!(String::from("Test1"), v.nombre);
    assert_eq!(String::from("Test2"), v.direccion);
    assert_eq!(5, v.id);
}

#[test]
fn test_veterinaria_agregar_cola() {
    let mut v = Veterinaria::new(String::from("Test1"), String::from("Test2"), 5);
    
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);

    assert_eq!(true, v.cola.is_empty());
    
    v.agregar_cola(m);

    assert_eq!(String::from("Test4"), v.cola[0].nombre);
}

#[test]
fn test_veterinaria_agregar_cola_prioridad() {
    let mut v = Veterinaria::new(String::from("Test1"), String::from("Test2"), 5);
    
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let d1 = Duenio::new(String::from("Test2"), String::from("Test3"), String::from("Test4"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);
    let m1 = Mascota::new(String::from("Test5"), 8, Animal::Gato, d1);

    v.agregar_cola(m);
    v.agregar_cola_prioridad(m1);

    assert_eq!(String::from("Test5"), v.cola[0].nombre);
}

#[test]
fn test_veterinaria_atender_prox() {
    let mut v = Veterinaria::new(String::from("Test1"), String::from("Test2"), 5);
    
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let d1 = Duenio::new(String::from("Test2"), String::from("Test3"), String::from("Test4"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);
    let m1 = Mascota::new(String::from("Test5"), 8, Animal::Gato, d1);

    v.agregar_cola(m);
    v.agregar_cola_prioridad(m1);

    let m2 = v.atender_prox();
    assert_eq!(String::from("Test5"), m2.unwrap().nombre);
    assert_eq!(1, v.cola.len());
}

#[test]
fn test_veterinaria_eliminar() {
    let mut v = Veterinaria::new(String::from("Test1"), String::from("Test2"), 5);
    
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let d1 = Duenio::new(String::from("Test2"), String::from("Test3"), String::from("Test4"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);
    let m1 = Mascota::new(String::from("Test5"), 8, Animal::Gato, d1.clone());

    v.agregar_cola(m);
    v.agregar_cola_prioridad(m1.clone());

    v.eliminar(&m1);

    assert_eq!(String::from("Test4"), v.cola[0].nombre);
    
}

#[test]
fn test_veterinaria_buscar_atencion() {
    let mut v = Veterinaria::new(String::from("Test1"), String::from("Test2"), 5);
    
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let d1 = Duenio::new(String::from("Test2"), String::from("Test3"), String::from("Test4"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);
    let m1 = Mascota::new(String::from("Test5"), 8, Animal::Gato, d1.clone());

    v.registrar_atencion(m, String::from("Test6"), String::from("Test7"), None);
    v.registrar_atencion(m1, String::from("Test8"), String::from("Test9"), Some(Fecha::new(20, 7, 2024)));

    assert_eq!(String::from("Test6"), v.buscar_atencion("Test4", "Test1", "Test3").unwrap().diagnostico);
    assert_eq!(None, v.buscar_atencion("Test200", "Test100", "Test23"));
}

#[test]
fn test_veterinaria_modificar_diagnostico() {
    let mut v = Veterinaria::new(String::from("Test1"), String::from("Test2"), 5);
    
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);

    v.registrar_atencion(m, String::from("Test6"), String::from("Test7"), None);

    v.modificar_diagnostico(&v.atenciones_realizadas[0].clone(), &String::from("Diagnostico"));
    assert_eq!(String::from("Diagnostico"), v.buscar_atencion("Test4", "Test1", "Test3").unwrap().diagnostico);
}

#[test]
fn test_veterinaria_modificar_fecha() {
    let mut v = Veterinaria::new(String::from("Test1"), String::from("Test2"), 5);
    
    let d = Duenio::new(String::from("Test1"), String::from("Test2"), String::from("Test3"));
    let m = Mascota::new(String::from("Test4"), 4, Animal::Perro, d);

    v.registrar_atencion(m, String::from("Test6"), String::from("Test7"), None);

    v.modificar_fecha(&v.atenciones_realizadas[0].clone(), Some(Fecha::new(10, 6, 2024)));
    assert_eq!(10, v.buscar_atencion("Test4", "Test1", "Test3").unwrap().visita.as_ref().unwrap().dia);
}
