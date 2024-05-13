pub struct _Persona {
    pub nombre: String,
    pub edad: i32,
    pub direccion: Option<String>
}

impl _Persona {
    pub fn _new(nombre: String, edad: i32, direccion: Option<String>) -> _Persona {
        _Persona {
            nombre,
            edad,
            direccion
        }
    }

    pub fn _to_string(&self) -> String {
        let string = match &self.direccion {
            Some(dir) => format!("Nombre: {} Edad: {} Direccion: {}", self.nombre, self.edad, dir),
            None => format!("Nombre: {} Edad: {}", self.nombre, self.edad)
        };

        string
    }

    pub fn _obtener_edad(&self) -> i32 {
        self.edad
    }

    pub fn _actualizar_direccion(&mut self, dir: Option<String>) {
        self.direccion = dir;
    }
}

#[test]
fn test_new() {
    let persona = _Persona::_new("test".to_string(), 23, Some("Calle 20".to_string()));
    
    assert_eq!(String::from("test"), persona.nombre);
    assert_eq!(23, persona.edad);
    assert_eq!(true, persona.direccion.is_some());
    assert_eq!(String::from("Calle 20"), persona.direccion.unwrap());
}
#[test]
fn test_to_string() {
    let persona = _Persona::_new("test".to_string(), 23, Some("Calle 20".to_string()));
    let persona1 = _Persona::_new("test".to_string(), 23, None);
    
    assert_eq!(String::from("Nombre: test Edad: 23 Direccion: Calle 20"), persona._to_string());
    assert_eq!(String::from("Nombre: test Edad: 23"), persona1._to_string());
}
#[test]
fn obtener_edad() {
    let persona = _Persona::_new("test".to_string(), 23, Some("Calle 20".to_string()));

    assert_eq!(23, persona.edad);
}
#[test]
fn test_actualizar_direccion() {
    let mut persona = _Persona::_new("test".to_string(), 23, Some("Calle 20".to_string()));

    persona._actualizar_direccion(Some(String::from("Calle 22")));
    assert_eq!(String::from("Calle 22"), persona.direccion.unwrap());
}