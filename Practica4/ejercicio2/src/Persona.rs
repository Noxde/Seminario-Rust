#[derive(PartialEq, Debug)]
struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}

impl<'a> Persona<'a> {
    pub fn new(nombre:&'a str, apellido:&'a str, direccion:&'a str, ciudad:&'a str, salario:f64, edad:u8) -> Persona<'a> {
        Persona { nombre, apellido, direccion, ciudad, salario, edad }
    }
}

pub fn mayor_salario<'a>(personas: &'a Vec<Persona>, x: f64) -> Vec<&'a Persona<'a>> {
    personas.iter().filter(|&p| p.salario > x).collect()
}

pub fn edad_ciudad<'a>(personas: &'a Vec<Persona>, edad: u8, ciudad: &str) -> Vec<&'a Persona<'a>> {
    personas.iter().filter(|&p| p.edad > edad && p.ciudad == ciudad).collect()
}

pub fn todos_ciudad<'a>(personas: &'a Vec<Persona>, ciudad: &str) -> bool {
    personas.iter().all(|p| p.ciudad.to_lowercase() == ciudad.to_lowercase())
}

pub fn alguien_ciudad<'a>(personas: &'a Vec<Persona>, ciudad: &str) -> bool {
    personas.iter().any(|p| p.ciudad.to_lowercase() == ciudad.to_lowercase())
}

pub fn existe<'a>(personas: &'a Vec<Persona>, persona: &Persona) -> bool {
    if let Some(_) = personas.iter().find(|&p| p == persona) {
        return true
    }
    false
}

pub fn retornar_edades(personas: &Vec<Persona>) -> Vec<u8> {
    personas.iter().map(|p| p.edad).collect()
}

pub fn mayor_menor_salarios<'a>(personas: &'a Vec<Persona>) -> Option<(&'a Persona<'a>, &'a Persona<'a>)> {
    if personas.is_empty() {
        return None;
    }
    let max = personas.iter().max_by(|x, y| x.salario.partial_cmp(&y.salario).unwrap_or(x.edad.cmp(&y.edad))).unwrap();

    let min = personas.iter().min_by(|x, y| x.salario.partial_cmp(&y.salario).unwrap_or(x.edad.cmp(&y.edad))).unwrap();

    Some((min, max))
    
}


#[test]
fn test_mayor_salario() {
    let personas = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Cordoba", 60000.0, 35),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Rosario", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Mendoza", 80000.0, 45),
        Persona::new("Pedro", "Gonzalez", "Calle 345", "La Plata", 90000.0, 50),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Mar del Plata", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Santa Fe", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "San Juan", 100000.0, 75),
    ];

    assert_eq!(vec![&Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55), &Persona::new("Sofia", "Lopez", "Calle 901", "Mar del Plata", 120000.0, 60), &Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65)], mayor_salario(&personas, 100000.0));
    assert_eq!(Vec::<&Persona>::new(), mayor_salario(&personas, 90000000.0));
}

#[test]
fn test_edad_ciudad() {
    let personas = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Tucuman", 60000.0, 35),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Rosario", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Mendoza", 80000.0, 45),
        Persona::new("Pedro", "Gonzalez", "Calle 345", "La Plata", 90000.0, 50),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Tucuman", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Tucuman", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "San Juan", 100000.0, 75),
    ];

    assert_eq!(vec![&Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55), &Persona::new("Sofia", "Lopez", "Calle 901", "Tucuman", 120000.0, 60), &Persona::new("Jorge", "Ramirez", "Calle 567", "Tucuman", 90000.0, 70)], edad_ciudad(&personas, 50, "Tucuman"));
    assert_eq!(Vec::<&Persona>::new(), edad_ciudad(&personas, 20, "Santa fe"));
}

#[test]
fn test_todos_ciudad() {
    let personas = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Buenos Aires", 60000.0, 35),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Buenos Aires", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Buenos Aires", 80000.0, 45),
        Persona::new("Pedro", "Gonzalez", "Calle 345", "Buenos Aires", 90000.0, 50),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Buenos Aires", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Buenos Aires", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Buenos Aires", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Buenos Aires", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "Buenos Aires", 100000.0, 75),
    ];

    let personas1 = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Tucuman", 60000.0, 35),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Rosario", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Mendoza", 80000.0, 45),
        Persona::new("Pedro", "Gonzalez", "Calle 345", "La Plata", 90000.0, 50),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Tucuman", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Tucuman", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "San Juan", 100000.0, 75),
    ];

    assert_eq!(true, todos_ciudad(&personas, "Buenos Aires"));
    assert_eq!(false, todos_ciudad(&personas1, "Rosario"));
}

#[test]
fn test_alguien_ciudad() {
    let personas = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Buenos Aires", 60000.0, 35),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Buenos Aires", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Buenos Aires", 80000.0, 45),
        Persona::new("Pedro", "Gonzalez", "Calle 345", "Buenos Aires", 90000.0, 50),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Buenos Aires", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Buenos Aires", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Buenos Aires", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Buenos Aires", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "Buenos Aires", 100000.0, 75),
    ];

    let personas1 = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Tucuman", 60000.0, 35),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Rosario", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Mendoza", 80000.0, 45),
        Persona::new("Pedro", "Gonzalez", "Calle 345", "La Plata", 90000.0, 50),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Tucuman", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Tucuman", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "San Juan", 100000.0, 75),
    ];

    assert_eq!(true, alguien_ciudad(&personas1, "San Juan"));
    assert_eq!(false, alguien_ciudad(&personas, "Tucuman"));
}

fn test_existe() {
    let personas = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Tucuman", 60000.0, 35),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Rosario", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Mendoza", 80000.0, 45),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Tucuman", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Tucuman", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "San Juan", 100000.0, 75),
    ];
    let buscar = Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30);
    let no_esta = Persona::new("Pedro", "Gonzalez", "Calle 345", "La Plata", 90000.0, 50);

    assert_eq!(true, existe(&personas, &buscar));
    assert_eq!(false, existe(&personas, &no_esta));
}

fn test_retornar_edades() {
    let personas = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Tucuman", 60000.0, 35),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Rosario", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Mendoza", 80000.0, 45),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Tucuman", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Tucuman", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "San Juan", 100000.0, 75),
    ];

    assert_eq!(vec![30, 35, 40, 45, 55, 60, 65, 70, 75], retornar_edades(&personas));
}

fn test_mayor_menor_salario() {
    let personas = vec![
        Persona::new("Juan", "Perez", "Calle 123", "Buenos Aires", 50000.0, 30),
        Persona::new("Maria", "Gomez", "Avenida 456", "Tucuman", 50000.0, 40),
        Persona::new("Carlos", "Rodriguez", "Calle 789", "Rosario", 70000.0, 40),
        Persona::new("Ana", "Martinez", "Avenida 012", "Mendoza", 80000.0, 45),
        Persona::new("Luis", "Fernandez", "Avenida 678", "Tucuman", 110000.0, 55),
        Persona::new("Sofia", "Lopez", "Calle 901", "Tucuman", 120000.0, 60),
        Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65),
        Persona::new("Jorge", "Ramirez", "Calle 567", "Tucuman", 90000.0, 70),
        Persona::new("Marta", "Sanchez", "Avenida 890", "San Juan", 100000.0, 75),
    ];

    let vacio: Vec<Persona> = Vec::new();

    assert_eq!((&Persona::new("Maria", "Gomez", "Avenida 456", "Tucuman", 50000.0, 40), &Persona::new("Gabriela", "Torres", "Avenida 234", "Salta", 130000.0, 65)), mayor_menor_salarios(&personas).unwrap());
    assert_eq!(None, mayor_menor_salarios(&vacio));

}