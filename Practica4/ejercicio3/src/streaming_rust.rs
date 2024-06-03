use std::collections::HashMap;
use chrono::*;
use crate::fecha::Fecha;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MedioDePago {
    Efectivo,
    MercadoPago,
    TarjetaCredito,
    Bancaria,
    Cripto
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SubscriptionType {
    Basic,
    Clasic,
    Super
}


#[derive(Clone, Debug, PartialEq)]
pub struct SubscriptionInfo {
    tipo: SubscriptionType,    
    mensual: f64,
    meses: u8,
    inicio: Fecha
}

pub struct Usuario {
    id: u32,
    sub: Option<SubscriptionInfo>,
    pago: MedioDePago,
}

pub struct Plataforma {
    usuarios: Vec<Usuario>,
}

impl Plataforma {
    pub fn new() -> Plataforma {
        Plataforma {
            usuarios: Vec::new(),
        }
    }

    pub fn crear_usuario(&mut self, id: u32, sub: Option<SubscriptionInfo>, pago: MedioDePago) {
        self.usuarios.push(Usuario::new(id, sub, pago));
    }

    pub fn buscar_usuario(&mut self, id: u32) -> Option<&mut Usuario> {
        if let Some(user) = self.usuarios.iter_mut().find(|x| x.id == id) {
            return Some(user);
        } else {
            None
        }
    }

    pub fn downgrade(&mut self, id: u32) -> bool {
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == id) {
            if  let Some(sub) = &mut user.sub {
                match &sub.tipo {
                    SubscriptionType::Super => {
                        sub.tipo = SubscriptionType::Clasic;
                        sub.mensual = 10.99;
                    },
                    SubscriptionType::Clasic => {
                        sub.tipo = SubscriptionType::Basic;
                        sub.mensual = 7.99;
                    }
                    SubscriptionType::Basic => {
                        user.sub = None;
                    }
                    
                };
                true
            } else {
                false // No sub
            }
        } else {
            false // Didnt find user
        }
    }

    pub fn upgrade(&mut self, id: u32) -> bool {
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == id) {
            if  let Some(sub) = &mut user.sub {
                match &sub.tipo {
                    SubscriptionType::Super => {
                        return false;
                    },
                    SubscriptionType::Clasic => {
                        sub.tipo = SubscriptionType::Super;
                        sub.mensual = 13.99;
                    }
                    SubscriptionType::Basic => {
                        sub.tipo = SubscriptionType::Clasic;
                        sub.mensual = 10.99;
                    }
                };
                true
            } else {
                false // No sub
            }
        } else {
            false // Didnt find user
        }
    }

    pub fn cancelar_usuario(&mut self, id: u32) -> bool {
        if let Some(user) = self.usuarios.iter_mut().find(|u| u.id == id) {
            user.sub = None;
            true
        } else {
            false
        }
    }

    pub fn mayor_metodo_activo(&self) -> Option<MedioDePago> {
     let mut map = HashMap::new();
        
        for u in &self.usuarios {
            if u.sub.as_ref().unwrap().es_activa() {
                *map.entry(u.pago.clone()).or_insert(0) += 1;
            }
        }

        map.into_iter().max_by_key(|&(_, count)| count).map(|(sub, _)| sub) 
    }

    pub fn mayor_suscripcion_activa(&self) -> Option<SubscriptionType> {
        let mut map = HashMap::new();
        
        for u in &self.usuarios {
            if u.sub.as_ref().unwrap().es_activa() {
                *map.entry(u.sub.as_ref().unwrap().tipo.clone()).or_insert(0) += 1;
            }
        }

        map.into_iter().max_by_key(|&(_, count)| count).map(|(sub, _)| sub)
    }

    pub fn mayor_metodo(&self) -> Option<MedioDePago> {
        let mut map = HashMap::new();
           
           for u in &self.usuarios {
                *map.entry(u.pago.clone()).or_insert(0) += 1;
           }
   
           map.into_iter().max_by_key(|&(_, count)| count).map(|(sub, _)| sub) 
       }
   
       pub fn mayor_suscripcion(&self) -> Option<SubscriptionType> {
        let mut map = HashMap::new();
        
        for u in &self.usuarios {
            *map.entry(u.sub.as_ref().unwrap().tipo.clone()).or_insert(0) += 1;
        }

        map.into_iter().max_by_key(|&(_, count)| count).map(|(sub, _)| sub)
    }

}

impl SubscriptionInfo {
    pub fn new(tipo: SubscriptionType, mensual: f64, meses: u8, inicio: Fecha) -> SubscriptionInfo {
        SubscriptionInfo {
            tipo,
            mensual,
            meses,
            inicio
        }
    }

    pub fn es_activa(&self) -> bool {
        let actual: DateTime<Utc> = Utc::now(); 
        let actual_fecha = Fecha::new(actual.day(), actual.month(), actual.year() as u32);
        let mut sumado = self.inicio.clone();
        sumado.sumar_dias(30 * self.meses as u32);

        if sumado.es_mayor(&actual_fecha) {
            true
        } else {
            false
        }
    
    }
}

impl Usuario {
    pub fn new(id: u32, sub: Option<SubscriptionInfo>, pago: MedioDePago) -> Usuario {
        Usuario {
            id,
            sub,
            pago
        }
    }
}

#[test]
pub fn test_plataforma_new() {
    let plataforma = Plataforma::new();
    assert_eq!(plataforma.usuarios.len(), 0);
}

#[test]
pub fn test_plataforma_crear_usuario() {
    let mut plataforma = Plataforma::new();
    plataforma.crear_usuario(1, None, MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(2,
    Some(SubscriptionInfo::new(
            SubscriptionType::Super,
            13.99,
            5,
            Fecha::new(15,
            2,
            2024))
        ), 
    MedioDePago::TarjetaCredito
    );
    
    assert_eq!(plataforma.usuarios.len(), 2);
    assert_eq!(plataforma.usuarios[0].id, 1);
    assert_eq!(plataforma.usuarios[0].sub, None);
    assert_eq!(plataforma.usuarios[0].pago, MedioDePago::TarjetaCredito);

    assert_eq!(plataforma.usuarios[1].id, 2);
    assert_eq!(plataforma.usuarios[1].sub.as_ref().unwrap().tipo, SubscriptionType::Super);
    assert_eq!(plataforma.usuarios[1].pago, MedioDePago::TarjetaCredito);
}

#[test]
pub fn test_plataforma_buscar_usuario() {
    let mut plataforma = Plataforma::new();
    plataforma.crear_usuario(1, None, MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(2,
        Some(SubscriptionInfo::new(
                SubscriptionType::Super,
                13.99,
                5,
                Fecha::new(15,
                2,
                2024))
            ), 
        MedioDePago::TarjetaCredito
        );
    plataforma.crear_usuario(3,
        Some(SubscriptionInfo::new(
                SubscriptionType::Clasic,
                10.99,
                3,
                Fecha::new(23,
                3,
                2024))
            ), 
        MedioDePago::TarjetaCredito
        );
    let user = plataforma.buscar_usuario(1);
    assert_eq!(user.unwrap().id, 1);

    let user = plataforma.buscar_usuario(2);
    assert_eq!(user.unwrap().id, 2);
    let user = plataforma.buscar_usuario(3);

    assert_eq!(user.unwrap().id, 3);
    let user = plataforma.buscar_usuario(4);
    assert!(user.is_none());


}

#[test]
pub fn test_plataforma_downgrade() {
    let mut plataforma = Plataforma::new();
    let sub_super = Some(SubscriptionInfo::new(SubscriptionType::Super, 13.99, 1, Fecha::new(10, 3, 2024)));
    let sub_basica = Some(SubscriptionInfo::new(SubscriptionType::Basic, 7.99, 1, Fecha::new(5, 4, 2024)));
    
    plataforma.crear_usuario(1, sub_super, MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(2, sub_basica, MedioDePago::Cripto);

    plataforma.downgrade(1);
    plataforma.downgrade(2);
    
    assert_eq!(plataforma.usuarios[0].sub.as_ref().unwrap().tipo, SubscriptionType::Clasic);
    assert_eq!(plataforma.usuarios[1].sub, None);
}

#[test]
pub fn test_plataforma_upgrade() {
    let mut plataforma = Plataforma::new();
    let sub_basica = Some(SubscriptionInfo::new(SubscriptionType::Basic, 7.99, 1, Fecha::new(2, 5, 2024)));
    let sub_super = Some(SubscriptionInfo::new(SubscriptionType::Super, 13.99, 3, Fecha::new(4, 3, 2024)));
   
    plataforma.crear_usuario(1, sub_basica, MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(2, sub_super, MedioDePago::TarjetaCredito);

    plataforma.upgrade(1);
    plataforma.upgrade(2);

    assert_eq!(plataforma.usuarios[0].sub.as_ref().unwrap().tipo, SubscriptionType::Clasic);
    assert_eq!(plataforma.usuarios[1].sub.as_ref().unwrap().tipo, SubscriptionType::Super);
}

#[test]
pub fn test_plataforma_cancelar_usuario() {
    let mut plataforma = Plataforma::new();
    let sub = Some(SubscriptionInfo::new(SubscriptionType::Basic, 7.99, 1, Fecha::new(1, 1, 2022)));
    
    plataforma.crear_usuario(1, sub, MedioDePago::TarjetaCredito);
    plataforma.cancelar_usuario(1);
    
    assert!(plataforma.usuarios[0].sub.is_none());
}

#[test]
pub fn test_plataforma_mayor_suscripcion_activa() {
    let mut plataforma = Plataforma::new();

    let sub_basic = Some(SubscriptionInfo::new(SubscriptionType::Basic, 7.99, 1, Fecha::new(5, 5, 2024)));
    let sub_clasic = Some(SubscriptionInfo::new(SubscriptionType::Clasic, 10.99, 3, Fecha::new(20, 3, 2024)));
    let sub_super = Some(SubscriptionInfo::new(SubscriptionType::Super, 13.99, 1, Fecha::new(15, 5, 2024)));
    let inactiva = Some(SubscriptionInfo::new(SubscriptionType::Clasic, 7.99, 1, Fecha::new(1, 4, 2024)));
    plataforma.crear_usuario(1, sub_basic.clone(), MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(2, sub_basic.clone(), MedioDePago::Efectivo);
    plataforma.crear_usuario(3, sub_clasic, MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(4, inactiva.clone(), MedioDePago::MercadoPago);
    plataforma.crear_usuario(5, inactiva, MedioDePago::MercadoPago);
    plataforma.crear_usuario(6, sub_super.clone(), MedioDePago::TarjetaCredito);

    assert_eq!(SubscriptionType::Basic, plataforma.mayor_suscripcion_activa().unwrap());

    let plataforma = Plataforma::new();
    assert_eq!(None, plataforma.mayor_suscripcion_activa());
}


#[test]
pub fn test_plataforma_mayor_suscripcion() {
    let mut plataforma = Plataforma::new();

    let sub_basic = Some(SubscriptionInfo::new(SubscriptionType::Basic, 7.99, 1, Fecha::new(5, 5, 2024)));
    let sub_clasic = Some(SubscriptionInfo::new(SubscriptionType::Clasic, 10.99, 3, Fecha::new(20, 3, 2024)));
    let sub_super = Some(SubscriptionInfo::new(SubscriptionType::Super, 13.99, 1, Fecha::new(15, 5, 2024)));
    let inactiva = Some(SubscriptionInfo::new(SubscriptionType::Clasic, 7.99, 1, Fecha::new(1, 4, 2024)));
    plataforma.crear_usuario(1, sub_basic.clone(), MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(2, sub_basic.clone(), MedioDePago::Efectivo);
    plataforma.crear_usuario(3, sub_clasic, MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(4, inactiva.clone(), MedioDePago::MercadoPago);
    plataforma.crear_usuario(5, inactiva, MedioDePago::MercadoPago);
    plataforma.crear_usuario(6, sub_super.clone(), MedioDePago::TarjetaCredito);

    assert_eq!(SubscriptionType::Clasic, plataforma.mayor_suscripcion().unwrap());

    let plataforma = Plataforma::new();
    assert_eq!(None, plataforma.mayor_suscripcion());
}

#[test]
pub fn test_plataforma_mayor_metodo_activo() {
    let mut plataforma = Plataforma::new();

    let sub_basic = Some(SubscriptionInfo::new(SubscriptionType::Basic, 7.99, 1, Fecha::new(5, 5, 2024)));
    let sub_clasic = Some(SubscriptionInfo::new(SubscriptionType::Clasic, 10.99, 3, Fecha::new(20, 3, 2024)));
    let sub_super = Some(SubscriptionInfo::new(SubscriptionType::Super, 13.99, 1, Fecha::new(15, 5, 2024)));
    let inactiva = Some(SubscriptionInfo::new(SubscriptionType::Clasic, 7.99, 1, Fecha::new(1, 4, 2024)));
    plataforma.crear_usuario(1, sub_basic.clone(), MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(2, sub_basic.clone(), MedioDePago::Efectivo);
    plataforma.crear_usuario(3, sub_clasic, MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(4, inactiva.clone(), MedioDePago::MercadoPago);
    plataforma.crear_usuario(5, inactiva, MedioDePago::MercadoPago);
    plataforma.crear_usuario(6, sub_super.clone(), MedioDePago::TarjetaCredito);

    assert_eq!(MedioDePago::TarjetaCredito, plataforma.mayor_metodo_activo().unwrap());

    let plataforma = Plataforma::new();
    assert_eq!(None, plataforma.mayor_metodo_activo());
}

#[test]
pub fn test_plataforma_mayor_metodo() {
    let mut plataforma = Plataforma::new();

    let sub_basic = Some(SubscriptionInfo::new(SubscriptionType::Basic, 7.99, 1, Fecha::new(5, 5, 2024)));
    let sub_clasic = Some(SubscriptionInfo::new(SubscriptionType::Clasic, 10.99, 3, Fecha::new(20, 3, 2024)));
    let sub_super = Some(SubscriptionInfo::new(SubscriptionType::Super, 13.99, 1, Fecha::new(15, 5, 2024)));
    let inactiva = Some(SubscriptionInfo::new(SubscriptionType::Clasic, 7.99, 1, Fecha::new(1, 4, 2024)));
    plataforma.crear_usuario(1, sub_basic.clone(), MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(2, sub_basic.clone(), MedioDePago::Efectivo);
    plataforma.crear_usuario(3, sub_clasic, MedioDePago::TarjetaCredito);
    plataforma.crear_usuario(4, inactiva.clone(), MedioDePago::MercadoPago);
    plataforma.crear_usuario(5, inactiva.clone(), MedioDePago::MercadoPago);
    plataforma.crear_usuario(6, inactiva.clone(), MedioDePago::MercadoPago);
    plataforma.crear_usuario(7, inactiva, MedioDePago::MercadoPago);
    plataforma.crear_usuario(8, sub_super.clone(), MedioDePago::TarjetaCredito);

    assert_eq!(MedioDePago::MercadoPago, plataforma.mayor_metodo().unwrap());

    let plataforma = Plataforma::new();
    assert_eq!(None, plataforma.mayor_metodo_activo());
}