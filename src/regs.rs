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

    pub fn new(value: u8) -> Self {
        let mut s = Self {
            irp: false,
            rp1: false,
            rp0: false,
            to: false,
            pd: false,
            z: false,
            dc: false,
            c: false,
        };
        s.set(value);
        s
    }

    pub fn set(&mut self, value: u8) {
        self.irp = if (value >> 7) & 1 == 1 { true } else { false };
        self.rp1 = if (value >> 6) & 1 == 1 { true } else { false };
        self.rp0 = if (value >> 5) & 1 == 1 { true } else { false };
        self.to = if (value >> 4) & 1 == 1 { true } else { false };
        self.pd = if (value >> 3) & 1 == 1 { true } else { false };
        self.z = if (value >> 2) & 1 == 1 { true } else { false };
        self.dc = if (value >> 1) & 1 == 1 { true } else { false };
        self.c = if (value >> 0) & 1 == 1 { true } else { false };
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::new(0b00011000)
    }
}

#[derive(Debug, Clone)]
pub struct Option {
    pub rbpu: bool,
    pub intedg: bool,
    pub t0cs: bool,
    pub t0se: bool,
    pub psa: bool,
    pub ps2: bool,
    pub ps1: bool,
    pub ps0: bool,
}

impl Option {
    pub fn value(&self) -> u8 {
        let mut value = 0;
        value |= if self.rbpu { 1 } else { 0 };
        value <<= 1;
        value |= if self.intedg { 1 } else { 0 };
        value <<= 1;
        value |= if self.t0cs { 1 } else { 0 };
        value <<= 1;
        value |= if self.t0se { 1 } else { 0 };
        value <<= 1;
        value |= if self.psa { 1 } else { 0 };
        value <<= 1;
        value |= if self.ps2 { 1 } else { 0 };
        value <<= 1;
        value |= if self.ps1 { 1 } else { 0 };
        value <<= 1;
        value |= if self.ps0 { 1 } else { 0 };
        value
    }

    pub fn new(value: u8) -> Self {
        let mut s = Self {
            rbpu: false,
            intedg: false,
            t0cs: false,
            t0se: false,
            psa: false,
            ps2: false,
            ps1: false,
            ps0: false,
        };
        s.set(value);
        s
    }

    pub fn set(&mut self, value: u8) {
        self.rbpu = if (value >> 7) & 1 == 1 { true } else { false };
        self.intedg = if (value >> 6) & 1 == 1 { true } else { false };
        self.t0cs = if (value >> 5) & 1 == 1 { true } else { false };
        self.t0se = if (value >> 4) & 1 == 1 { true } else { false };
        self.psa = if (value >> 3) & 1 == 1 { true } else { false };
        self.ps2 = if (value >> 2) & 1 == 1 { true } else { false };
        self.ps1 = if (value >> 1) & 1 == 1 { true } else { false };
        self.ps0 = if (value >> 0) & 1 == 1 { true } else { false };
    }
}

impl Default for Option {
    fn default() -> Self {
        Self::new(0xff)
    }
}

#[derive(Debug, Clone)]
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

    pub fn new(value: u8) -> Self {
        let mut s = Self {
            gie: false,
            peie: false,
            tmr0ie: false,
            inte: false,
            rbie: false,
            tmr0if: false,
            intf: false,
            rbif: false,
        };
        s.set(value);
        s
    }

    pub fn set(&mut self, value: u8) {
        self.gie = if (value >> 7) & 1 == 1 { true } else { false };
        self.peie = if (value >> 6) & 1 == 1 { true } else { false };
        self.tmr0ie = if (value >> 5) & 1 == 1 { true } else { false };
        self.inte = if (value >> 4) & 1 == 1 { true } else { false };
        self.rbie = if (value >> 3) & 1 == 1 { true } else { false };
        self.tmr0if = if (value >> 2) & 1 == 1 { true } else { false };
        self.intf = if (value >> 1) & 1 == 1 { true } else { false };
        self.rbif = if (value >> 0) & 1 == 1 { true } else { false };
    }
}

impl Default for Intcon {
    fn default() -> Self {
        Self::new(0x00)
    }
}

#[derive(Debug, Clone)]
pub struct PIE1 {
    pub pspie: bool,
    pub adie: bool,
    pub rcie: bool,
    pub txie: bool,
    pub sspie: bool,
    pub ccp1ie: bool,
    pub tmr2ie: bool,
    pub tmr1ie: bool,
}

impl PIE1 {
    pub fn value(&self) -> u8 {
        let mut value = 0;
        value |= if self.pspie { 1 } else { 0 };
        value <<= 1;
        value |= if self.adie { 1 } else { 0 };
        value <<= 1;
        value |= if self.rcie { 1 } else { 0 };
        value <<= 1;
        value |= if self.txie { 1 } else { 0 };
        value <<= 1;
        value |= if self.sspie { 1 } else { 0 };
        value <<= 1;
        value |= if self.ccp1ie { 1 } else { 0 };
        value <<= 1;
        value |= if self.tmr2ie { 1 } else { 0 };
        value <<= 1;
        value |= if self.tmr1ie { 1 } else { 0 };
        value
    }

    pub fn new(value: u8) -> Self {
        let mut s = Self {
            pspie: false,
            adie: false,
            rcie: false,
            txie: false,
            sspie: false,
            ccp1ie: false,
            tmr2ie: false,
            tmr1ie: false,
        };
        s.set(value);
        s
    }

    pub fn set(&mut self, value: u8) {
        self.pspie = if (value >> 7) & 1 == 1 { true } else { false };
        self.adie = if (value >> 6) & 1 == 1 { true } else { false };
        self.rcie = if (value >> 5) & 1 == 1 { true } else { false };
        self.txie = if (value >> 4) & 1 == 1 { true } else { false };
        self.sspie = if (value >> 3) & 1 == 1 { true } else { false };
        self.ccp1ie = if (value >> 2) & 1 == 1 { true } else { false };
        self.tmr2ie = if (value >> 1) & 1 == 1 { true } else { false };
        self.tmr1ie = if (value >> 0) & 1 == 1 { true } else { false };
    }
}

impl Default for PIE1 {
    fn default() -> Self {
        Self::new(0x00)
    }
}

#[derive(Debug, Clone)]
pub struct PIR1 {
    pub pspif: bool,
    pub adif: bool,
    pub rcif: bool,
    pub txif: bool,
    pub sspif: bool,
    pub ccp1if: bool,
    pub tmr2if: bool,
    pub tmr1if: bool,
}

impl PIR1 {
    pub fn value(&self) -> u8 {
        let mut value = 0;
        value |= if self.pspif { 1 } else { 0 };
        value <<= 1;
        value |= if self.adif { 1 } else { 0 };
        value <<= 1;
        value |= if self.rcif { 1 } else { 0 };
        value <<= 1;
        value |= if self.txif { 1 } else { 0 };
        value <<= 1;
        value |= if self.sspif { 1 } else { 0 };
        value <<= 1;
        value |= if self.ccp1if { 1 } else { 0 };
        value <<= 1;
        value |= if self.tmr2if { 1 } else { 0 };
        value <<= 1;
        value |= if self.tmr1if { 1 } else { 0 };
        value
    }

    pub fn new(value: u8) -> Self {
        let mut s = Self {
            pspif: false,
            adif: false,
            rcif: false,
            txif: false,
            sspif: false,
            ccp1if: false,
            tmr2if: false,
            tmr1if: false,
        };
        s.set(value);
        s
    }

    pub fn set(&mut self, value: u8) {
        self.pspif = if (value >> 7) & 1 == 1 { true } else { false };
        self.adif = if (value >> 6) & 1 == 1 { true } else { false };
        self.rcif = if (value >> 5) & 1 == 1 { true } else { false };
        self.txif = if (value >> 4) & 1 == 1 { true } else { false };
        self.sspif = if (value >> 3) & 1 == 1 { true } else { false };
        self.ccp1if = if (value >> 2) & 1 == 1 { true } else { false };
        self.tmr2if = if (value >> 1) & 1 == 1 { true } else { false };
        self.tmr1if = if (value >> 0) & 1 == 1 { true } else { false };
    }
}

impl Default for PIR1 {
    fn default() -> Self {
        Self::new(0x00)
    }
}

#[derive(Debug, Clone)]
pub struct T1CON {
    pub t1ckps1: bool,
    pub t1ckps0: bool,
    pub t1oscen: bool,
    pub t1sync: bool,
    pub tmr1cs: bool,
    pub tmr1on: bool,
}

impl T1CON {
    pub fn value(&self) -> u8 {
        let mut value = 0;
        value |= if self.t1ckps1 { 1 } else { 0 };
        value <<= 1;
        value |= if self.t1ckps0 { 1 } else { 0 };
        value <<= 1;
        value |= if self.t1oscen { 1 } else { 0 };
        value <<= 1;
        value |= if self.t1sync { 1 } else { 0 };
        value <<= 1;
        value |= if self.tmr1cs { 1 } else { 0 };
        value <<= 1;
        value |= if self.tmr1on { 1 } else { 0 };
        value
    }

    pub fn new(value: u8) -> Self {
        let mut s = Self {
            t1ckps1: false,
            t1ckps0: false,
            t1oscen: false,
            t1sync: false,
            tmr1cs: false,
            tmr1on: false,
        };
        s.set(value);
        s
    }

    pub fn set(&mut self, value: u8) {
        self.t1ckps1 = if (value >> 5) & 1 == 1 { true } else { false };
        self.t1ckps0 = if (value >> 4) & 1 == 1 { true } else { false };
        self.t1oscen = if (value >> 3) & 1 == 1 { true } else { false };
        self.t1sync = if (value >> 2) & 1 == 1 { true } else { false };
        self.tmr1cs = if (value >> 1) & 1 == 1 { true } else { false };
        self.tmr1on = if (value >> 0) & 1 == 1 { true } else { false };
    }
}

impl Default for T1CON {
    fn default() -> Self {
        Self::new(0x00)
    }
}
