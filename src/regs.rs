#[derive(Debug, Clone)]
pub struct Status {
    pub irp: bool,
    pub rp1: bool,
    pub rp0: bool,
    pub to: bool,
    pub pd: bool,
    pub z: bool,
    pub dc: bool,
    pub c: bool,
}
impl Status {
    pub fn value(&self) -> u8 {
        let mut value = 0;
        value |= if self.irp { 1 } else { 0 };
        value <<= 1;
        value |= if self.rp1 { 1 } else { 0 };
        value <<= 1;
        value |= if self.rp0 { 1 } else { 0 };
        value <<= 1;
        value |= if self.to { 1 } else { 0 };
        value <<= 1;
        value |= if self.pd { 1 } else { 0 };
        value <<= 1;
        value |= if self.z { 1 } else { 0 };
        value <<= 1;
        value |= if self.dc { 1 } else { 0 };
        value <<= 1;
        value |= if self.c { 1 } else { 0 };
        value
    }
}

impl Default for Status {
    fn default() -> Self {
        Self {
            irp: false,
            rp1: false,
            rp0: false,
            to: true,
            pd: true,
            z: false,
            dc: false,
            c: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Intcon {
    pub gie: bool,
    pub peie: bool,
    pub tmr0ie: bool,
    pub inte: bool,
    pub rbie: bool,
    pub tmr0if: bool,
    pub intf: bool,
    pub rbif: bool,
}

impl Intcon {
    pub fn value(&self) -> u8 {
        let mut value = 0;
        value |= if self.gie { 1 } else { 0 };
        value <<= 1;
        value |= if self.peie { 1 } else { 0 };
        value <<= 1;
        value |= if self.tmr0ie { 1 } else { 0 };
        value <<= 1;
        value |= if self.inte { 1 } else { 0 };
        value <<= 1;
        value |= if self.rbie { 1 } else { 0 };
        value <<= 1;
        value |= if self.tmr0if { 1 } else { 0 };
        value <<= 1;
        value |= if self.intf { 1 } else { 0 };
        value <<= 1;
        value |= if self.rbif { 1 } else { 0 };
        value
    }
}
