#[derive(Debug, PartialEq, Clone)]
pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub anio: u32
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, anio: u32) -> Fecha {
        Fecha {
            dia,
            mes,
            anio
        }
    }

    pub fn es_bisiesto(&self) -> bool {
        if self.anio % 100 == 0 && (self.anio / 100) % 4 == 0 {
            true
        } else {
            self.anio % 4 == 0
        }
    }

    pub fn es_fecha_valida(&self) -> bool {
        let dias30 = [4,6,9,11];

        if (self.mes > 12 || self.mes == 0) || (self.dia == 0 || self.dia > 31) || self.anio == 0 {
            return false
        }

        if self.mes == 2 && self.dia > 28 && !self.es_bisiesto() {
            return false
        } 

        if dias30.contains(&self.mes) && self.dia == 31 {
            return false
        }

        true
    }

    pub fn sumar_dias(&mut self, dias: u32) {
        let dias30 = [4,6,9,11];

        for _i in 0..dias {
            self.dia += 1;
            // manage cases
            if (dias30.contains(&self.mes) && self.dia == 31) || self.dia == 32 {
                self.dia = 1;
                self.mes += 1;
            }
            // manage february
            if (self.mes == 2 && self.es_bisiesto() && self.dia == 30) || 
               (self.mes == 2 && !self.es_bisiesto() && self.dia == 29) {
                self.dia = 1;
                self.mes += 1;
            }

            // manage going to the previous year
            if self.mes == 13 {
                self.anio += 1;
                self.mes = 1;
                self.dia = 1;
            }
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        let dias30 = [4,6,9,11];

        for _i in 0..dias {
            self.dia -= 1;
            // manage cases
            if self.dia == 0 {
                self.mes -= 1;
                if dias30.contains(&self.mes) {
                    self.dia = 30;
                } else {
                    self.dia = 31;
                }

                // manage february
                if self.mes == 2 {
                    if self.es_bisiesto() {
                        self.dia = 29;
                    } else {
                        self.dia = 28
                    }
                }

                // manage going to the previous year
                if self.mes == 0 {
                    self.anio -= 1;
                    self.mes = 12;
                    self.dia = 31;
                }
            }
        }
    }

    pub fn es_mayor(&self, f: Fecha) -> bool {
        if !f.es_fecha_valida() {
            return true;
        }

        if self.anio < f.anio ||
           self.anio == f.anio && self.mes < f.mes ||
           self.anio == f.anio && self.mes == f.mes && self.dia < f.dia {
            false
        } else {
            true
        }
    }
}

#[test]
fn test_new() {
    let f = Fecha::new(15, 6, 2010);

    assert_eq!(15, f.dia);
    assert_eq!(6, f.mes);
    assert_eq!(2010, f.anio);
}
#[test]
fn test_es_fecha_valida() {
    let f = Fecha::new(200, 30, 1000);
    let f2 = Fecha::new(20, 12, 2000);

    assert_eq!(false, f.es_fecha_valida());
    assert_eq!(true, f2.es_fecha_valida());
}
#[test]
fn test_es_bisiesto() {
    let bisiesto = Fecha::new(29, 2, 2024);
    let no_bisiesto = Fecha::new(10, 1, 2025);

    assert_eq!(true, bisiesto.es_bisiesto());
    assert_ne!(true, no_bisiesto.es_bisiesto());
}
#[test]
fn test_restar_dias() {
    let mut f = Fecha::new(17, 1, 2004);
    let mut bisiesto = Fecha::new(3, 3, 2024);
    let mut mes = Fecha::new(23, 5, 2024);

    f.restar_dias(17);
    bisiesto.restar_dias(3);
    mes.restar_dias(31);

    assert_eq!(Fecha::new(31, 12, 2003), f);
    assert_eq!(Fecha::new(29, 2, 2024), bisiesto);
    assert_eq!(Fecha::new(22, 4, 2024), mes);
}

#[test]
fn test_sumar_dias() {
    let mut f = Fecha::new(17, 1, 2004);
    let mut bisiesto = Fecha::new(25, 2, 2024);
    let mut fin = Fecha::new(30, 12, 2024);

    f.sumar_dias(3);
    bisiesto.sumar_dias(5);
    fin.sumar_dias(10);

    assert_eq!(Fecha::new(20, 1, 2004), f);
    assert_eq!(Fecha::new(1, 3, 2024), bisiesto);
    assert_eq!(Fecha::new(9, 1, 2025), fin);
}
#[test]
fn test_es_mayor() {
    let f = Fecha::new(17, 1, 2004);
    let f2 = Fecha::new(29, 2, 2024);
    let f3 = Fecha::new(30, 4, 2020);

    assert_eq!(false, f.es_mayor(Fecha::new(17, 1, 2005)));
    assert_eq!(true, f2.es_mayor(Fecha::new(29, 1, 2024)));
    assert_eq!(true, f3.es_mayor(Fecha::new(15, 4, 2020)));
}