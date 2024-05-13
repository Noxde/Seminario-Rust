#[derive(Debug, PartialEq, Clone)]
pub enum Genero {
    Rock,
    Pop,
    Jazz,
    Otros
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

pub struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>
}

impl Cancion {
    pub fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion {
            titulo,
            artista,
            genero,
        }
    }
}

impl Playlist {
    pub fn new(nombre: String) -> Playlist {
        Playlist {
            nombre,
            canciones: Vec::new()
        }
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self, cancion: &Cancion) {
        let mut index = None;
        for (i, c) in self.canciones.iter().enumerate() {
            if c == cancion {
                index = Some(i);
            }
        }
        if index.is_some() {
            self.canciones.remove(index.unwrap());
        }
    }

    pub fn mover_cancion(&mut self, cancion: &Cancion, pos: usize) {
        if pos - 1 > self.canciones.len() {
            return;
        }
        let mut index: Option<usize> = None;
        for (i, c) in self.canciones.iter().enumerate() {
            if c == cancion {
                index = Some(i);
            }
        }

        if index.is_some() {
            self.canciones.swap(index.unwrap(), pos - 1);
        }
    }

    pub fn buscar_cancion_nombre(&self, nombre: String) -> Option<Cancion> {
        for cancion in self.canciones.iter() {
            if cancion.titulo == nombre {
                return Some(cancion.clone());
            }
        }

        None
    }

    pub fn obtener_canciones_genero(&self, genero: Genero) -> Vec<Cancion> {
        let mut canciones: Vec<Cancion> = Vec::new();
        
        for cancion in self.canciones.iter() {
            if cancion.genero == genero {
                canciones.push(cancion.clone())
            }
        }

        canciones
    }

    pub fn obtener_canciones_artista(&self, artista: String) -> Vec<Cancion> {
        let mut canciones: Vec<Cancion> = Vec::new();
        
        for cancion in self.canciones.iter() {
            if cancion.artista == artista {
                canciones.push(cancion.clone())
            }
        }

        canciones
    }

    pub fn modificar_titulo(&mut self, titulo: String) {
        self.nombre = titulo;
    }

    pub fn eliminar_canciones(&mut self) {
        self.canciones = Vec::new();
    }
}

#[test]
fn test_cancion_new() {
    let c = Cancion::new(String::from("Test1"), String::from("Test2"), Genero::Otros);

    assert_eq!(String::from("Test1"), c.titulo);
    assert_eq!(String::from("Test2"), c.artista);
    assert_eq!(Genero::Otros, c.genero);
}

#[test]
fn test_playlist_new() {
    let p = Playlist::new(String::from("Test1"));

    assert_eq!(String::from("Test1"), p.nombre);
    assert_eq!(true, p.canciones.is_empty());    
}

#[test]
fn test_playlist_agregar_cancion() {
    let mut p = Playlist::new(String::from("Test1"));
    let c = Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros);
    
    assert_eq!(true, p.canciones.is_empty());    
    p.agregar_cancion(c);
    
    assert_eq!(1, p.canciones.len());
    assert_eq!(String::from("Test2"), p.canciones[0].titulo);
}

#[test]
fn test_playlist_eliminar_cancion() {
    let mut p = Playlist::new(String::from("Test1"));
    let c = Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros);
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros));

    p.eliminar_cancion(&c);

    assert_eq!(true, p.canciones.is_empty());
}

#[test]
fn test_playlist_mover_cancion() {
    let mut p = Playlist::new(String::from("Test1"));
    let c = Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros);
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test5"), Genero::Pop));
    
    p.mover_cancion(&c, 2);
    assert_eq!(c, p.canciones[1]);
    assert_ne!(c, p.canciones[0]);
}

#[test]
fn test_playlist_buscar_nombre() {
    let mut p = Playlist::new(String::from("Test1"));
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test5"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test6"), String::from("Test7"), Genero::Jazz));
    
    let c = p.buscar_cancion_nombre(String::from("Test6"));
    let c2 = p.buscar_cancion_nombre(String::from("No existe"));
    assert_eq!(p.canciones[2], c.unwrap());
    assert_eq!(None, c2);
}

#[test]
fn test_playlist_obtener_genero() {
    let mut p = Playlist::new(String::from("Test1"));
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Jazz));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test5"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test6"), String::from("Test7"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test8"), String::from("Test9"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test10"), String::from("Test11"), Genero::Jazz));
    p.agregar_cancion(Cancion::new(String::from("Test12"), String::from("Test13"), Genero::Rock));
    
    let rock = p.obtener_canciones_genero(Genero::Rock);
    let pop = p.obtener_canciones_genero(Genero::Pop);
    assert_eq!(3, rock.len());
    assert_eq!(true, pop.is_empty());
}

#[test]
fn test_playlist_modificar_titulo() {
    let mut p = Playlist::new(String::from("Test1"));

    p.modificar_titulo(String::from("Playlist musica"));

    assert_eq!(String::from("Playlist musica"), p.nombre);
}

#[test]
fn test_playlist_eliminar_canciones() {
    let mut p = Playlist::new(String::from("Test1"));
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Jazz));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test5"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test6"), String::from("Test7"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test8"), String::from("Test9"), Genero::Rock));

    p.eliminar_canciones();
    assert_eq!(true, p.canciones.is_empty());
}