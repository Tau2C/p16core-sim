pub mod mem;
pub mod regs;

use std::ops::Shl;

use circular_buffer::CircularBuffer;

use crate::{mem::Ram, regs::*};

#[derive(Debug, Clone)]
pub struct P16Core {
    program: [u16; 4096],
    file: Ram,
    skip_next: bool,
    stack: CircularBuffer<8, u16>,

    pub w: u8,
    status: Status,
    pc: u16,
    tmr0: u8,
    fsr: u8,
    port_a: u8,
    port_b: u8,
    port_c: u8,
    port_d: u8,
    pclath: u8,
    intcon: Intcon,
    pir1: u8,
    pie1: u8,
    indf1: u8,
    indf2: u8,
    t1con: u8,
    t1_l: u8,
    t1_h: u8,
    dan: u8,
    dseg: u8,
    rcsta: u8,
    tx_reg: u8,
    rc_reg: u8,
    ptr1_h: u8,
    ptr1_l: u8,
    ptr2_l: u8,
    ptr2_h: u8,
}

impl Default for P16Core {
    fn default() -> Self {
        Self {
            program: [0; 4096],
            file: Default::default(),
            skip_next: Default::default(),
            stack: CircularBuffer::new(),

            w: Default::default(),
            status: Default::default(),
            pc: Default::default(),
            tmr0: Default::default(),
            fsr: Default::default(),
            port_a: Default::default(),
            port_b: Default::default(),
            port_c: Default::default(),
            port_d: Default::default(),
            pclath: Default::default(),
            intcon: Default::default(),
            pir1: Default::default(),
            pie1: Default::default(),
            indf1: Default::default(),
            indf2: Default::default(),
            t1con: Default::default(),
            t1_l: Default::default(),
            t1_h: Default::default(),
            dan: Default::default(),
            dseg: Default::default(),
            rcsta: Default::default(),
            tx_reg: Default::default(),
            rc_reg: Default::default(),
            ptr1_h: Default::default(),
            ptr1_l: Default::default(),
            ptr2_l: Default::default(),
            ptr2_h: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Bit {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

impl TryFrom<u16> for Bit {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bit::B0),
            1 => Ok(Bit::B1),
            2 => Ok(Bit::B2),
            3 => Ok(Bit::B3),
            4 => Ok(Bit::B4),
            5 => Ok(Bit::B5),
            6 => Ok(Bit::B6),
            7 => Ok(Bit::B7),
            _ => Err(()),
        }
    }
}

impl Shl<Bit> for u8 {
    type Output = u8;

    fn shl(self, rhs: Bit) -> Self::Output {
        self << rhs as Self::Output
    }
}

impl Shl<Bit> for u16 {
    type Output = u16;

    fn shl(self, rhs: Bit) -> Self::Output {
        self << rhs as Self::Output
    }
}

impl Shl<Bit> for u32 {
    type Output = u32;

    fn shl(self, rhs: Bit) -> Self::Output {
        self << rhs as Self::Output
    }
}

impl Shl<Bit> for u64 {
    type Output = u64;

    fn shl(self, rhs: Bit) -> Self::Output {
        self << rhs as Self::Output
    }
}

impl Into<u32> for Bit {
    fn into(self) -> u32 {
        self as u32
    }
}

impl From<Bit> for u8 {
    fn from(b: Bit) -> u8 {
        b as u8
    }
}

type F = u8;
type D = bool;
type B = Bit;
type K = u8;
type A = u16;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    ADDWF { reg: F, dest: D },
    ANDWF { reg: F, dest: D },
    CLRF { reg: F },
    CLRW,
    COMF { reg: F, dest: D },
    DECF { reg: F, dest: D },
    DECFSZ { reg: F, dest: D },
    INCF { reg: F, dest: D },
    INCFSZ { reg: F, dest: D },
    IORWF { reg: F, dest: D },
    MOVF { reg: F, dest: D },
    MOVWF { reg: F },
    NOP,
    RLF { reg: F, dest: D },
    RRF { reg: F, dest: D },
    SUBWF { reg: F, dest: D },
    SWAPF { reg: F, dest: D },
    XORWF { reg: F, dest: D },
    BCF { reg: F, bit: B },
    BSF { reg: F, bit: B },
    BTFSC { reg: F, bit: B },
    BTFSS { reg: F, bit: B },
    ADDLW { lit: K },
    ANDLW { lit: K },
    CALL { lit: A },
    GOTO { lit: A },
    IORLW { lit: K },
    MOVLW { lit: K },
    RETFIE,
    RETLW { lit: K },
    RETURN,
    SUBLW { lit: K },
    XORLW { lit: K },
}

impl P16Core {
    pub fn new(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("failed to read file");

        let mut program = [0; 4096]; // 8K program memory
        let mut upper_addr = 0u32;

        for line in contents.lines() {
            if !line.starts_with(':') {
                continue;
            }
            let bytes = hex::decode(&line[1..]).unwrap(); // requires `hex` crate

            let count = bytes[0] as usize;
            let addr = ((bytes[1] as u16) << 8 | bytes[2] as u16) as u32;
            let rtype = bytes[3];
            let data = &bytes[4..4 + count];

            match rtype {
                0x00 => {
                    let full_addr = (upper_addr << 16) + addr;
                    for i in (0..count).step_by(2) {
                        let lo = data[i];
                        let hi = if i + 1 < count { data[i + 1] } else { 0 };
                        let word = ((hi as u16) << 8) | lo as u16;
                        program[(full_addr as usize + i) / 2] = word & 0x3FFF;
                    }
                }
                0x04 => {
                    upper_addr = ((data[0] as u32) << 8) | data[1] as u32;
                }
                0x01 => break, // EOF
                _ => {}
            }
        }

        Self {
            program,
            ..Default::default()
        }
    }

    fn decode(word: u16) -> Instruction {
        match (word >> 8) as u8 {
            0b00_0000..=0b00_1111 => match (word >> 8) as u8 {
                0b00_0000 => match (word & 0xff) as u8 {
                    0b0000_0000 | 0b0010_0000 | 0b0100_0000 | 0b0110_0000 => Instruction::NOP,
                    0b0000_1000 => Instruction::RETURN,
                    0b0110_0100 => {
                        panic!("CLRWDT OPCODE {word}")
                    }
                    0b0000_1001 => Instruction::RETFIE,
                    0b0110_0011 => {
                        panic!("SLEEP OPCODE {word}")
                    }
                    0b1000_0000..=0b1111_1111 => Instruction::MOVWF {
                        reg: (word & 0b0111_1111) as u8,
                    },
                    0b0000_0001..=0b0000_0111
                    | 0b0000_1010..=0b0001_1111
                    | 0b0010_0001..=0b0011_1111
                    | 0b0100_0000..=0b0101_1111
                    | 0b0110_0001..=0b0110_0010
                    | 0b0110_0101..=0b0111_1111 => {
                        panic!("UNKNOWN OPCODE {word}")
                    }
                },
                0b00_0001 => match (word >> 7) & 1 == 0 {
                    true => Instruction::CLRW,
                    false => Instruction::CLRF {
                        reg: (word & 0b0111_1111) as u8,
                    },
                },
                0b00_0010 => Instruction::SUBWF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_0011 => Instruction::DECF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_0100 => Instruction::IORWF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_0101 => Instruction::ANDWF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_0110 => Instruction::XORWF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_0111 => Instruction::ADDWF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_1000 => Instruction::MOVF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_1001 => Instruction::COMF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_1010 => Instruction::INCF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_1011 => Instruction::DECFSZ {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_1100 => Instruction::RRF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_1101 => Instruction::RLF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_1110 => Instruction::SWAPF {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b00_1111 => Instruction::INCFSZ {
                    reg: (word & 0b0111_1111) as u8,
                    dest: (word >> 7) & 1 == 1,
                },
                0b01_0000..=0b11_1111 => panic!("Unexpected OPCODE in 0b00_... section"),
                0b0100_0000..=0b1111_1111 => unreachable!(),
            },
            0b01_0000..=0b01_1111 => {
                let reg = (word & 0b0111_1111) as u8;
                let bit = Bit::try_from((word >> 7) & 0b0111).unwrap();
                match word >> 10 {
                    0b01_00 => Instruction::BCF { reg, bit },
                    0b01_01 => Instruction::BSF { reg, bit },
                    0b01_10 => Instruction::BTFSC { reg, bit },
                    0b01_11 => Instruction::BTFSS { reg, bit },
                    _ => unreachable!(),
                }
            }
            0b10_0000..=0b10_1111 => {
                let lit = word & 0b0111_1111_1111;
                match (word >> 11) & 1 == 0 {
                    true => Instruction::CALL { lit },
                    false => Instruction::GOTO { lit },
                }
            }
            0b11_0000..=0b11_1111 => match ((word >> 8) & 0b1111) as u8 {
                0b1001 => Instruction::ANDLW {
                    lit: (word & 0xff) as u8,
                },
                0b1110..=0b1111 => Instruction::ADDLW {
                    lit: (word & 0xff) as u8,
                },
                0b1000 => Instruction::IORLW {
                    lit: (word & 0xff) as u8,
                },
                0b0000..=0b0011 => Instruction::MOVLW {
                    lit: (word & 0xff) as u8,
                },
                0b0100..=0b0111 => Instruction::RETLW {
                    lit: (word & 0xff) as u8,
                },
                0b1100..=0b1101 => Instruction::SUBLW {
                    lit: (word & 0xff) as u8,
                },
                0b1010 => Instruction::XORLW {
                    lit: (word & 0xff) as u8,
                },
                0b1011 => panic!("UNKNOWN OPCODE {word}"),
                0b0001_0000..=0b1111_1111 => unreachable!(),
            },
            0b0100_0000..=0b1111_1111 => unreachable!(),
        }
    }

    fn get_next_op(&mut self) -> u16 {
        let op = if self.skip_next {
            0
        } else {
            self.program[(self.pc % 4096) as usize]
        };
        (self.pc, _) = self.pc.overflowing_add(1);
        self.skip_next = false;
        return op;
    }

    fn exec_op(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADDWF { reg, dest } => {
                let b = self.read(reg);
                let (result, c) = self.w.overflowing_add(b);

                self.status.z = result == 0;
                self.status.c = c;
                self.status.dc = ((self.w & 0x0F) + (b & 0x0F)) > 0x0F;

                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::ANDWF { reg, dest } => {
                let result = self.w & self.read(reg);

                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }

                self.status.z = self.w == 0;
            }
            Instruction::CLRF { reg } => {
                self.write(reg, 0);
                self.status.z = true;
            }
            Instruction::CLRW => {
                self.w = 0;
                self.status.z = true;
            }
            Instruction::COMF { reg, dest } => {
                let result = !self.read(reg);
                self.status.z = result == 0;
                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::DECF { reg, dest } => {
                let result = self.read(reg) - 1;
                self.status.z = result == 0;
                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::DECFSZ { reg, dest } => {
                let result = self.read(reg) - 1;
                if result == 0 {
                    self.skip_next = true;
                }
                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::INCF { reg, dest } => {
                let result = self.read(reg) + 1;
                self.status.z = result == 0;
                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::INCFSZ { reg, dest } => {
                let result = self.read(reg) + 1;
                if result == 0 {
                    self.skip_next = true;
                }
                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::IORWF { reg, dest } => {
                let result = self.w | self.read(reg);
                self.status.z = result == 0;
                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::MOVF { reg, dest } => {
                let f = self.read(reg);

                if dest {
                    self.write(reg, f);
                } else {
                    self.w = f;
                }
            }
            Instruction::MOVWF { reg } => {
                self.write(reg, self.w);
            }
            Instruction::NOP => {}
            Instruction::RLF { reg, dest } => {
                let v = self.read(reg);

                let c = v >> 7;
                let result = (v << 1) | self.status.c as u8;
                self.status.c = c == 1;

                if dest {
                    self.write(reg, result);
                } else {
                    self.w = v;
                }
            }
            Instruction::RRF { reg, dest } => {
                let v = self.read(reg);

                let c = v & 1;
                let result = (v >> 1) | ((self.status.c as u8) << 7);
                self.status.c = c == 1;

                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::SUBWF { reg, dest } => {
                let f = self.read(reg);
                let (result, c) = f.overflowing_sub(self.w);

                self.status.z = result == 0;
                self.status.c = c;
                self.status.dc = (f & 0x0F) >= (self.w & 0x0F);

                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::SWAPF { reg, dest } => {
                let f = self.read(reg);
                let result = ((f & 0xF0) >> 4) | ((f & 0xF) << 4);

                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::XORWF { reg, dest } => {
                let f = self.read(reg);
                let result = self.w ^ f;

                self.status.z = result == 0;

                if dest {
                    self.write(reg, result);
                } else {
                    self.w = result;
                }
            }
            Instruction::BCF { reg, bit } => {
                let result = self.read(reg) & !(1u8 << bit);
                self.write(reg, result);
            }
            Instruction::BSF { reg, bit } => {
                let result = self.read(reg) | (1u8 << bit);
                self.write(reg, result);
            }
            Instruction::BTFSC { reg, bit } => {
                let bit = self.read(reg) & (1u8 << bit) == 1;
                if !bit {
                    self.skip_next = true;
                }
            }
            Instruction::BTFSS { reg, bit } => {
                let bit = self.read(reg) & (1u8 << bit) == 1;
                if bit {
                    self.skip_next = true;
                }
            }
            Instruction::ADDLW { lit } => {
                let (result, c) = self.w.overflowing_add(lit);

                self.status.z = result == 0;
                self.status.c = c;
                self.status.dc = ((self.w & 0x0F) + (lit & 0x0F)) > 0x0F;

                self.w = result;
            }
            Instruction::ANDLW { lit } => {
                self.w = self.w & lit;
                self.status.z = self.w == 0;
            }
            Instruction::CALL { lit } => {
                self.stack.push_front(self.pc);
                self.pc = (self.pclath as u16 & 0x18 << 7) | lit & 0x3FF;
            }
            Instruction::GOTO { lit } => {
                self.pc = (self.pclath as u16 & 0x18 << 7) | lit & 0x3FF;
            }
            Instruction::IORLW { lit } => {
                self.w = self.w | lit;
                self.status.z = self.w == 0;
            }
            Instruction::MOVLW { lit } => {
                self.w = lit;
            }
            Instruction::RETFIE => {
                self.pc = self.stack.pop_front().unwrap();
                self.intcon.gie = true;
            }
            Instruction::RETLW { lit } => {
                self.w = lit;
                self.pc = self.stack.pop_front().unwrap();
            }
            Instruction::RETURN => {
                self.pc = self.stack.pop_front().unwrap();
            }
            Instruction::SUBLW { lit } => {
                let (result, c) = lit.overflowing_sub(self.w);

                self.status.z = result == 0;
                self.status.c = c;
                self.status.dc = (lit & 0x0F) >= (self.w & 0x0F);

                self.w = result;
            }
            Instruction::XORLW { lit } => {
                self.w = self.w ^ lit;
                self.status.z = self.w == 0;
            }
        }
    }

    fn write(&mut self, address: u8, value: u8) {
        let address =
            (((self.status.rp1 as u16) << 1 | (self.status.rp0 as u16)) << 8) | (address as u16);
        self.file.write(address, value);
    }

    fn read(&self, address: u8) -> u8 {
        let address =
            (((self.status.rp1 as u16) << 1 | (self.status.rp0 as u16)) << 8) | (address as u16);
        match address {
            0x000 | 0x080 | 0x100 | 0x180 => todo!("Indirect addr."), // Indirect addr
            0x001 | 0x101 => self.tmr0,                               // TMR0
            0x081 | 0x181 => self.tmr0,                               // OPTION_REG
            0x002 | 0x082 | 0x102 | 0x182 => (self.pc & 0xff) as u8,  // PCL
            0x003 | 0x083 | 0x103 | 0x183 => self.status.value(),     // STATUS
            0x004 | 0x084 | 0x104 | 0x184 => self.fsr,                // FSR
            0x005 => self.port_a,                                     // PORTA
            0x006 => self.port_b,                                     // PORTB
            0x007 => self.port_c,                                     // PORTC
            0x008 => self.port_d,                                     // PORTD
            0x009 => panic!("UNASIGNED ADDRESS {address}"),           // UNASIGNED
            0x00A => self.pclath,                                     // PCLATH
            0x00B | 0x08B | 0x10B | 0x18B => self.intcon.value(),     // INTCON
            0x00C => self.pir1,                                       // PIR1
            0x08C => self.pie1,                                       // PIE1
            0x00D => panic!("UNASIGNED ADDRESS {address}"),           // UNASIGNED
            0x00E | 0x08E | 0x10E | 0x18E => self.indf1,              // INDF1
            0x00F | 0x08F | 0x10F | 0x18F => self.indf2,              // INDF2
            0x010 => self.t1con,                                      // T1CON
            0x011 => self.t1_l,                                       // T1L
            0x012 => self.t1_h,                                       // T1H
            0x013 => self.dan,                                        // DAN
            0x014 => self.dseg,                                       // DSEG
            0x015..=0x017 => panic!("UNASIGNED ADDRESS {address}"),   // UNASIGNED
            0x018 => self.rcsta,                                      // RCSTA
            0x019 => self.tx_reg,                                     // TXREG
            0x01A => self.rc_reg,                                     // RCREG
            0x01B => panic!("UNASIGNED ADDRESS {address}"),           // UNASIGNED
            0x01C => self.ptr1_l,                                     // PTR1L
            0x01D => self.ptr1_h,                                     // PTR1H
            0x01E => self.ptr2_l,                                     // PTR2L
            0x01F => self.ptr2_h,                                     // PTR2H

            0x085..=0x08A => panic!("UNASIGNED ADDRESS {address}"), // UNASIGNED
            0x08D => panic!("UNASIGNED ADDRESS {address}"),         // UNASIGNED
            0x090..=0x09F => panic!("UNASIGNED ADDRESS {address}"), // UNASIGNED
            0x105..=0x10A => panic!("UNASIGNED ADDRESS {address}"), // UNASIGNED
            0x10C..=0x10D => panic!("UNASIGNED ADDRESS {address}"), // UNASIGNED
            0x185..=0x18A => panic!("UNASIGNED ADDRESS {address}"), // UNASIGNED
            0x18C..=0x18D => panic!("UNASIGNED ADDRESS {address}"), // UNASIGNED

            // NORMAL RAM
            0x020..=0x06f | 0x0A0..=0x0EF | 0x120..=0x16f | 0x1A0..=0x1EF => {
                self.file.read(address)
            }

            // EXTENDED RAM
            0x110..=0x11F | 0x190..=0x19F => self.file.read(address),

            // SHARED RAM
            0x070..=0x07f | 0x0f0..=0x0ff | 0x170..=0x17f | 0x1f0..=0x1ff => {
                self.file.read(0x070 | (address & 0xf))
            }

            0x200..=u16::MAX => unreachable!(),
        }
    }
}

fn main() {
    let file = "test.hex";
    let mut p16 = P16Core::new(file);
    loop {
        let op = p16.get_next_op();
        let instruction = P16Core::decode(op);
        println!("{:?}", instruction);
        p16.exec_op(instruction);
    }
}
