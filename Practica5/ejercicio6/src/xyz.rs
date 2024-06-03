
use std::{borrow::Borrow, collections::HashMap, fs::File, io::Write};
use rand::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};


use crate::fecha::Fecha;

#[derive(Clone, PartialEq)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum Prefijo {
    BTC,
    ETH,
    LTC, // Litecoin
    XRP, // Ripple
    BCH, // Bitcoin Cash
    ADA, // Cardano
}

#[derive(Clone)]
pub struct CriptoMoneda {
    valor: f64,
    blockchains: Vec<Blockchain>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    validado: bool,
    fiat: f64,
    balances: HashMap<Prefijo, f64>
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Medio {
    MercadoPago,
    TransferenciaBancaria
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Transaccion {
    IngresoDinero { fecha: Fecha, usuario: Usuario, monto: f64 },
    CompraCripto { fecha: Fecha, usuario: Usuario, cripto: Prefijo, cotizacion: f64, monto: f64 }, // Cotizacion esta dentro de cripto
    VentaCripto { fecha: Fecha, usuario: Usuario, cripto: Prefijo, cotizacion: f64, monto: f64 },
    RetiroCripto { fecha: Fecha, usuario: Usuario, blockchain: String, hash: String, cripto: Prefijo, cotizacion: f64, monto: f64 }, // Cotizacion esta dentro de cripto
    RecepcionCripto { fecha: Fecha, usuario: Usuario, blockchain: String, cripto: Prefijo, cotizacion: f64, monto: f64 }, // Cotizacion esta dentro de cripto
    RetiroFiat { fecha: Fecha, usuario: Usuario, monto: f64, medio: Medio } // Cotizacion esta dentro de cripto
}

pub struct XYZ {
    usuarios: Vec<Usuario>,
    transacciones: Vec<Transaccion>
}

fn get_fecha_actual() -> Fecha {
    let actual: DateTime<Utc> = Utc::now(); 
    Fecha::new(actual.day(), actual.month(), actual.year() as u32)
}


impl XYZ {
    pub fn new() -> XYZ {
        XYZ {
            usuarios: Vec::new(),
            transacciones: Vec::new()
        }
    }

    fn escribir_archivos(&self) {
        self.escribir_balances();
        self.escribir_transacciones();
    }

    fn escribir_balances(&self) {
        match File::create("./balances.json") {
            Ok(mut file) => {
                let b_s = serde_json::to_string_pretty(&self.usuarios).unwrap();
                file.write_all(&b_s.as_bytes()).expect("Error al escribir el archivo balances.json");
            },
            Err(error) => {
                println!("Error al crear archivo: {error}");
            }
        };
    }
    

    fn escribir_transacciones(&self) {
        match File::create("./transacciones.json") {
            Ok(mut file) => {
                let t_s = serde_json::to_string_pretty(&self.transacciones).unwrap();
                file.write_all(&t_s.as_bytes()).expect("Error al escribir el archivo transacciones.json");
            },
            Err(error) => {
                println!("Error al crear archivo: {error}");
            }
        };
    }
    

    pub fn crear_usuario(&mut self, nombre: String, apellido: String, email: String, dni: String) -> bool {
        let user = self.usuarios.iter_mut().find(|u| u.dni == dni);

        if user.is_some() {
            false // Usuario ya existe
        } else {
            self.usuarios.push(Usuario::new(nombre, apellido, email, dni));
            true
        }
    }

    pub fn validar_usuario(&mut self, dni: String) -> bool {
        let user = self.usuarios.iter_mut().find(|u| u.dni == dni);

        if let Some(u) = user {
            u.validado = true;
            true
        } else {
            false
        }
    }

    pub fn ingresar_dinero(&mut self, dni: String, monto: f64) -> bool {
        let user = self.usuarios.iter_mut().find(|u| u.dni == dni);
        
        if let Some(u) = user {
            if !u.validado {
               return false // No esta validado
            }

            u.fiat += monto;
            self.transacciones.push(Transaccion::IngresoDinero { fecha: get_fecha_actual(), usuario: u.clone(), monto });
            self.escribir_archivos();
            true
        } else {
            false // No esta el usuario
        }
    }

    pub fn comprar_cripto(&mut self, dni: String, cripto: Prefijo, monto_cripto: f64) -> bool {
        let user = self.usuarios.iter_mut().find(|u| u.dni == dni);

        if let Some(u) = user {
            let precio = monto_cripto * cripto.cotizacion();
            if !u.validado || u.fiat < precio {
                return false; // No esta validado/No hay balance suficiente
            }
            
            let info = cripto.get_info();

            *u.balances.entry(cripto.clone()).or_insert(0.0) += monto_cripto;
            u.fiat -= precio;
            self.transacciones.push(Transaccion::CompraCripto { fecha: get_fecha_actual(), usuario: u.clone(), cripto, cotizacion: info.valor,  monto: monto_cripto });
            self.escribir_archivos();
            true
        } else {
            false // No se encontro el usuario
        }
    }

    pub fn vender_cripto(&mut self, dni: String, cripto: Prefijo, monto_cripto: f64) -> bool {
        let user = self.usuarios.iter_mut().find(|u| u.dni == dni);

        if let Some(u) = user {
            if !u.validado {
                return false;
            }

            if let Some(bal) = u.balances.get(&cripto) {
                if *bal < monto_cripto {
                    return false; // No hay balance suficiente 
                }
                let info = cripto.get_info();

                *u.balances.entry(cripto.clone()).or_insert(0.0) -= monto_cripto;
                u.fiat += monto_cripto / cripto.cotizacion() ;
                self.transacciones.push(Transaccion::VentaCripto { fecha: get_fecha_actual(), usuario: u.clone(), cripto, cotizacion: info.valor, monto: monto_cripto });
                self.escribir_archivos();
                true
            } else {
                false // No tiene la cripto especificada
            }
        } else {
            false // No se encontro el usuario
        }
    }

    pub fn retirar_blockchain(&mut self, dni: String, cripto: Prefijo, blockchain: String, monto_cripto: f64) -> bool {
        let user = self.usuarios.iter_mut().find(|u| u.dni == dni);

        if let Some(u) = user {
            if !u.validado {
                return false; // No esta validado
            }

            if let Some(bal) = u.balances.get_mut(&cripto) {
                if *bal < monto_cripto {
                    return false // Balance insuficiente
                }

                let info = cripto.get_info();
                
                if let Some(bl) = info.blockchains.iter().find(|x| &x.nombre == &blockchain) {
                    let hash = format!("{}#{}", bl.nombre, rand::random::<u32>());
                    
                    *bal -= monto_cripto;
                    self.transacciones.push(Transaccion::RetiroCripto { fecha: get_fecha_actual(), usuario: u.clone(), blockchain: bl.nombre.clone(), hash, cripto, cotizacion: info.valor, monto: monto_cripto });
                    self.escribir_archivos();
                    true
                } else {
                    false // Blockchain no valido
                }
            } else {
                false // No tiene la cripto especificada
            }
        } else {
            false // No es encontro el usuario
        }
    }

    pub fn recepcion_blockchain(&mut self, dni: String, cripto: Prefijo, blockchain: String, monto_cripto: f64) -> bool {
        let user = self.usuarios.iter_mut().find(|u| u.dni == dni);

        if let Some(u) = user {
            if !u.validado {
                return false;
            }
            let info = cripto.get_info();
            
            *u.balances.entry(cripto.clone()).or_insert(0.0) += monto_cripto;
            self.transacciones.push(Transaccion::RecepcionCripto { fecha: get_fecha_actual(), usuario: u.clone(), blockchain: blockchain, cripto, cotizacion: info.valor, monto: monto_cripto });
            self.escribir_archivos();
            true
        } else {
            false // No se encontro al usuario
        }
    }

    pub fn retirar_fiat(&mut self, dni: String, medio: Medio, monto_fiat: f64) -> bool {
        let user = self.usuarios.iter_mut().find(|u| u.dni == dni);

        if let Some(u) = user {
            if !u.validado {
                return false;
            }
            if u.fiat < monto_fiat {
                return false // No hay balance suficiente
            }

            u.fiat -= monto_fiat;
            self.transacciones.push(Transaccion::RetiroFiat { fecha: get_fecha_actual(), usuario: u.clone(), monto: monto_fiat, medio });
            self.escribir_archivos();
            true
        } else {
            false // No se encontro al usuario
        }
    }

    pub fn top_cripto_ventas(&self) -> Option<Prefijo> {
        let mut map: HashMap<Prefijo, u32> = HashMap::new();
            
            for t in &self.transacciones {
                match t {
                    Transaccion::VentaCripto { cripto, .. } => *map.entry(cripto.clone()).or_insert(0) += 1,
                    _ => ()
                }
            }

            map.into_iter().max_by_key(|&(_, count)| count).map(|(pr, _)| pr) 
    }

    pub fn top_cripto_compras(&self) -> Option<Prefijo> {
        let mut map: HashMap<Prefijo, u32> = HashMap::new();
            
            for t in &self.transacciones {
                match t {
                    Transaccion::CompraCripto { cripto, ..} => *map.entry(cripto.clone()).or_insert(0) += 1,
                    _ => ()
                }
            }

            map.into_iter().max_by_key(|&(_, count)| count).map(|(pr, _)| pr) 
    }

    pub fn top_volumen_ventas(&self) -> Option<Prefijo> {
        let mut map: HashMap<Prefijo, f64> = HashMap::new();
            
        for t in &self.transacciones {
            match t {
                Transaccion::VentaCripto { cripto, monto, ..} => *map.entry(cripto.clone()).or_insert(0.0) += monto,
                _ => ()
            }
        }

        map.into_iter().max_by(|(_, totala), (_, totalb)| totala.partial_cmp(totalb).unwrap_or(std::cmp::Ordering::Equal)).map(|(pr, _)| pr)
    }

    pub fn top_volumen_compras(&self) -> Option<Prefijo> {
        let mut map: HashMap<Prefijo, f64> = HashMap::new();
            
        for t in &self.transacciones {
            match t {
                Transaccion::CompraCripto { cripto, monto, ..} => *map.entry(cripto.clone()).or_insert(0.0) += monto,
                _ => ()
            }
        }

        map.into_iter().max_by(|(_, totala), (_, totalb)| totala.partial_cmp(totalb).unwrap_or(std::cmp::Ordering::Equal)).map(|(pr, _)| pr)
    }
}

impl Prefijo {
    fn cotizacion(&self) -> f64 {
        match  self {
            Prefijo::BTC => 45000.0,
            Prefijo::ETH => 3000.0,
            Prefijo::LTC => 200.0,
            Prefijo::XRP => 1.0,
            Prefijo::BCH => 500.0,
            Prefijo::ADA => 1.23
        }
    } 

    fn get_info(&self) -> CriptoMoneda {
        match self {
            Prefijo::BTC => CriptoMoneda { valor: self.cotizacion(), blockchains: vec![Blockchain{nombre: "Bitcoin".to_string(), prefijo: "BTC".to_string()}]},
            Prefijo::ETH => CriptoMoneda { valor: self.cotizacion(), blockchains: vec![Blockchain{nombre: "Ethereum".to_string(), prefijo: "ETH".to_string()}]},
            Prefijo::LTC => CriptoMoneda { valor: self.cotizacion(), blockchains: vec![Blockchain{nombre: "Litecoin".to_string(), prefijo: "LTC".to_string()}]},
            Prefijo::XRP => CriptoMoneda { valor: self.cotizacion(), blockchains: vec![Blockchain{nombre: "Ripple".to_string(), prefijo: "XRP".to_string()}]},
            Prefijo::BCH => CriptoMoneda { valor: self.cotizacion(), blockchains: vec![Blockchain{nombre: "Bitcoin Cash".to_string(), prefijo: "BCH".to_string()}]},
            Prefijo::ADA => CriptoMoneda { valor: self.cotizacion(), blockchains: vec![Blockchain{nombre: "Cardano".to_string(), prefijo: "ADA".to_string()}]},
        }
    }
}

impl Usuario {
    pub fn new(nombre: String, apellido: String, email: String, dni: String) -> Usuario {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            validado: false,
            fiat: 0.0,
            balances: HashMap::new()
        }
    }
}

#[test]
fn test_usuario_new() {
    let user = Usuario::new("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());

    assert_eq!("John".to_string(), user.nombre);
    assert_eq!("Doe".to_string(), user.apellido);
    assert_eq!("John.doe@gmail.com".to_string(), user.email);
    assert_eq!("123123123".to_string(), user.dni);
}

#[test]
fn test_xyz_new() {
    let plataforma = XYZ::new();

    assert!(plataforma.transacciones.is_empty());
    assert!(plataforma.usuarios.is_empty());
}

#[test]
fn test_xyz_crear_usuario() {
    let mut plataforma = XYZ::new();

    assert!(plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string()));

    assert_eq!(1, plataforma.usuarios.len());
    assert_eq!("John".to_string(), plataforma.usuarios[0].nombre);
    assert_eq!(false, plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string()));
}

#[test]
fn test_xyz_validar_usuario() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());

    assert!(plataforma.validar_usuario("123123123".to_string()));
    assert_eq!(true, plataforma.usuarios[0].validado);
    assert_eq!(false, plataforma.validar_usuario("no existe".to_string()));

}

#[test]
fn test_xyz_ingresar_dinero() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());

    assert_eq!(false, plataforma.ingresar_dinero("123123123".to_string(), 200.0));
    plataforma.validar_usuario("123123123".to_string());
    assert!(plataforma.ingresar_dinero("123123123".to_string(), 200.0));

    match plataforma.transacciones[0] {
        Transaccion::IngresoDinero { .. } => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(200.0, plataforma.usuarios[0].fiat);

    assert_eq!(false, plataforma.ingresar_dinero("no existe".to_string(), 200.0));

}

#[test]
fn test_xyz_comprar_cripto() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    plataforma.validar_usuario("123123123".to_string());

    assert_eq!(false, plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 1.0));
    plataforma.ingresar_dinero("123123123".to_string(), 50000.0);
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 1.0));

    println!("{:?}", plataforma.transacciones[0]);

    match plataforma.transacciones[1] {
        Transaccion::CompraCripto { .. } => assert!(true),
        _ => assert!(false)
    }

    assert_eq!(&1.0, plataforma.usuarios[0].balances.get(&Prefijo::BTC).unwrap());

    assert_eq!(false, plataforma.comprar_cripto("no existe".to_string(), Prefijo::BTC, 1.0));
}

#[test]
fn test_xyz_vender_cripto() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    assert_eq!(false, plataforma.vender_cripto("no existe".to_string(), Prefijo::BTC, 1.0)); // no existe
    
    assert_eq!(false, plataforma.vender_cripto("123123123".to_string(), Prefijo::BTC, 1.0)); // sin validar
    plataforma.validar_usuario("123123123".to_string());
    assert_eq!(false, plataforma.vender_cripto("123123123".to_string(), Prefijo::BTC, 1.0)); // sin cripto

    plataforma.ingresar_dinero("123123123".to_string(), 100000.0);
    plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 1.5);

    assert_eq!(false, plataforma.vender_cripto("123123123".to_string(), Prefijo::BTC, 5.0)); // sin balance suficiente

    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::BTC, 1.0));

    match plataforma.transacciones[2] {
        Transaccion::VentaCripto { .. } => assert!(true),
        _ => assert!(false)
    }


    assert_eq!(32500.000022222222, plataforma.usuarios[0].fiat);
    assert_eq!(&0.5, plataforma.usuarios[0].balances.get(&Prefijo::BTC).unwrap());
}

#[test]
fn test_xyz_retirar_blockchain() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    assert_eq!(false, plataforma.retirar_blockchain("123123123".to_string(), Prefijo::BTC, "Bitcoin".to_string(), 0.2)); // sin validar
    assert_eq!(false, plataforma.retirar_blockchain("no existe".to_string(), Prefijo::BTC, "Bitcoin".to_string(), 0.2)); // no existe usuario
    

    plataforma.validar_usuario("123123123".to_string());
    assert_eq!(false, plataforma.retirar_blockchain("123123123".to_string(), Prefijo::BTC, "Bitcoin".to_string(), 0.2)); // sin cripto

    plataforma.ingresar_dinero("123123123".to_string(), 100000.0);
    plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 1.5);

    assert_eq!(false, plataforma.retirar_blockchain("123123123".to_string(), Prefijo::BTC, "No existe".to_string(), 0.2));
    assert_eq!(false, plataforma.retirar_blockchain("123123123".to_string(), Prefijo::BTC, "Bitcoin".to_string(), 10.0));
    assert!(plataforma.retirar_blockchain("123123123".to_string(), Prefijo::BTC, "Bitcoin".to_string(), 0.2));

    match plataforma.transacciones[2] {
        Transaccion::RetiroCripto { .. } => assert!(true),
        _ => assert!(false)
    }


    assert_eq!(&1.3, plataforma.usuarios[0].balances.get(&Prefijo::BTC).unwrap());
}

#[test]
fn test_xyz_recepcion_blockchain() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    assert_eq!(false, plataforma.recepcion_blockchain("no existe".to_string(), Prefijo::BTC, "Bitcoin".to_string(), 0.03)); // sin validar
    assert_eq!(false, plataforma.recepcion_blockchain("123123123".to_string(), Prefijo::BTC, "Bitcoin".to_string(), 0.03)); // sin validar

    plataforma.validar_usuario("123123123".to_string());

    assert!(plataforma.recepcion_blockchain("123123123".to_string(), Prefijo::BTC, "Bitcoin".to_string(), 0.03));

    match plataforma.transacciones[0] {
        Transaccion::RecepcionCripto { .. } => assert!(true),
        _ => assert!(false)
    }


    assert_eq!(&0.03, plataforma.usuarios[0].balances.get(&Prefijo::BTC).unwrap());
}

#[test]
fn test_xyz_retirar_fiat() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    plataforma.validar_usuario("123123123".to_string());
    plataforma.ingresar_dinero("123123123".to_string(), 100000.0);
    
    assert!(plataforma.retirar_fiat("123123123".to_string(), Medio::TransferenciaBancaria, 75000.0));

    match plataforma.transacciones[1] {
        Transaccion::RetiroFiat { .. } => assert!(true),
        _ => assert!(false)
    }


    assert_eq!(25000.0, plataforma.usuarios[0].fiat);
}

#[test]
fn test_xyz_top_cripto_compras() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    plataforma.validar_usuario("123123123".to_string());
    plataforma.ingresar_dinero("123123123".to_string(), 1000000.0);
    
    assert_eq!(None, plataforma.top_cripto_compras());


    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 2.0));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::ETH, 10.0));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.1));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.3));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.08));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::XRP, 300000.0));

    assert_eq!(Prefijo::BTC, plataforma.top_cripto_compras().unwrap());
}

#[test]
fn test_xyz_top_cripto_ventas() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    plataforma.validar_usuario("123123123".to_string());
    plataforma.ingresar_dinero("123123123".to_string(), 1000000.0);
    
    assert_eq!(None, plataforma.top_cripto_ventas());


    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 2.0));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::ETH, 10.0));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.1));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.3));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.08));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::XRP, 300000.0));

    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::ETH, 1.0));
    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::BTC, 0.5));
    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::ETH, 4.0));
    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::ETH, 2.0));
    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::XRP, 100000.0));


    assert_eq!(Prefijo::ETH, plataforma.top_cripto_ventas().unwrap());
}

#[test]
fn test_xyz_top_volumen_compras() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    plataforma.validar_usuario("123123123".to_string());
    plataforma.ingresar_dinero("123123123".to_string(), 1000000.0);
    
    assert_eq!(None, plataforma.top_volumen_compras());


    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 2.0));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::ETH, 10.0));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.1));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.3));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.08));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::XRP, 300000.0));

    assert_eq!(Prefijo::XRP, plataforma.top_volumen_compras().unwrap());
}

#[test]
fn test_xyz_top_volumen_ventas() {
    let mut plataforma = XYZ::new();
    plataforma.crear_usuario("John".to_string(), "Doe".to_string(), "John.doe@gmail.com".to_string(), "123123123".to_string());
    plataforma.validar_usuario("123123123".to_string());
    plataforma.ingresar_dinero("123123123".to_string(), 1000000.0);
    
    assert_eq!(None, plataforma.top_volumen_ventas());


    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 2.0));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::ETH, 10.0));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.1));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.3));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::BTC, 0.08));
    assert!(plataforma.comprar_cripto("123123123".to_string(), Prefijo::XRP, 300000.0));

    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::ETH, 1.0));
    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::BTC, 0.5));
    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::ETH, 4.0));
    assert!(plataforma.vender_cripto("123123123".to_string(), Prefijo::ETH, 2.0));


    assert_eq!(Prefijo::ETH, plataforma.top_volumen_ventas().unwrap());
}