pub struct Examen {
    nombre: String,
    nota: f64
}

pub struct Estudiante {
    nombre: String,
    id: u32,
    calificaciones: Option<Vec<Examen>>
}

impl Examen {
    pub fn new(nombre: String, nota: f64) -> Examen {
        Examen {
            nombre,
            nota
        }
    }
}

impl Estudiante {
    pub fn new(nombre: String, id: u32, calificaciones: Option<Vec<Examen>>) -> Estudiante {
        Estudiante {
            nombre,
            id,
            calificaciones
        }
    }

    pub fn obtener_promedio(&self) -> f64 {
        if self.calificaciones.is_none() {
            return 0.0;
        }


        let n = self.calificaciones.as_ref().unwrap().len();
        let mut total = 0.0;
        for e in self.calificaciones.as_ref().unwrap() {
            total += e.nota;
        }
        total / n as f64
    }

    pub fn obtener_calificacion_mas_alta(&self) -> f64 {
        if self.calificaciones.is_none() {
            return 0.0;
        }

        let mut max = 0.0;
        for e in self.calificaciones.as_ref().unwrap() {
            if e.nota > max {
                max = e.nota;
            }
        }
        max
    }

    pub fn obtener_calificacion_mas_baja(&self) -> f64 {
        if self.calificaciones.is_none() {
            return 0.0;
        }

        let mut min = 999.9;
        for e in self.calificaciones.as_ref().unwrap() {
            if e.nota < min {
                min = e.nota;
            }
        }
        min
    }
}

#[test]
fn test_new_estudiante() {
    let e = Estudiante::new("Test".to_string(), 20, None);

    assert_eq!("Test".to_string(), e.nombre);
    assert_eq!(20, e.id);
}
#[test]
fn test_new_examen() {
    let e = Examen::new("TestMateria".to_string(), 6.0);

    assert_eq!("TestMateria".to_string(), e.nombre);
    assert_eq!(6.0, e.nota);
}
#[test]
fn test_obtener_promedio() {
    let mut calificaciones: Vec<Examen> = Vec::new();
    calificaciones.push(Examen::new(String::from("Test1"), 5.0));
    calificaciones.push(Examen::new(String::from("Test2"), 7.0));
    calificaciones.push(Examen::new(String::from("Test3"), 5.5));
    calificaciones.push(Examen::new(String::from("Test4"), 8.25));
    let e = Estudiante::new("Test".to_string(), 20, Some(calificaciones));
    
    assert_eq!(6.4375, e.obtener_promedio());
}

#[test]
fn test_calificacion_mas_alta() {
    let mut calificaciones: Vec<Examen> = Vec::new();
    calificaciones.push(Examen::new(String::from("Test1"), 5.0));
    calificaciones.push(Examen::new(String::from("Test2"), 7.0));
    calificaciones.push(Examen::new(String::from("Test3"), 5.5));
    calificaciones.push(Examen::new(String::from("Test4"), 8.25));
    let e = Estudiante::new("Test".to_string(), 20, Some(calificaciones));

    assert_eq!(8.25, e.obtener_calificacion_mas_alta());
}

#[test]
fn test_calificacion_mas_baja() {
    let mut calificaciones: Vec<Examen> = Vec::new();
    calificaciones.push(Examen::new(String::from("Test1"), 5.0));
    calificaciones.push(Examen::new(String::from("Test2"), 7.0));
    calificaciones.push(Examen::new(String::from("Test3"), 5.5));
    calificaciones.push(Examen::new(String::from("Test4"), 8.25));
    let e = Estudiante::new("Test".to_string(), 20, Some(calificaciones));

    assert_eq!(5.0, e.obtener_calificacion_mas_baja());
}