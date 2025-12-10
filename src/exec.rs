use crate::P16Core;
use std::ops::Shl;

#[derive(Debug, Clone, Copy)]
pub enum Bit {
    B0 = 0,
    B1 = 1,
    B2 = 2,
    B3 = 3,
    B4 = 4,
    B5 = 5,
    B6 = 6,
    B7 = 7,
}

impl Bit {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
    pub fn as_u16(self) -> u16 {
        self as u16
    }
    pub fn as_u32(self) -> u32 {
        self as u32
    }
    pub fn as_u64(self) -> u64 {
        self as u64
    }
}

macro_rules! impl_try_from_int_for_bit {
    ($($t:ty),*) => {
        $(
            impl TryFrom<$t> for Bit {
                type Error = ();

                fn try_from(value: $t) -> Result<Self, Self::Error> {
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
        )*
    };
}

impl_try_from_int_for_bit!(u8, u16, u32, u64, i8, i16, i32, i64);

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

#[tracing::instrument]
pub fn decode(word: u16) -> Instruction {
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

#[tracing::instrument(skip(core))]
pub fn exec_op(core: &mut P16Core, instruction: Instruction) {
    match instruction {
        Instruction::ADDWF { reg, dest } => {
            let b = core.read(reg as u16);
            let (result, c) = core.w.overflowing_add(b);

            core.status.z = result == 0;
            core.status.c = c;
            core.status.dc = ((core.w & 0x0F) + (b & 0x0F)) > 0x0F;

            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::ANDWF { reg, dest } => {
            let result = core.w & core.read(reg as u16);

            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }

            core.status.z = core.w == 0;
        }
        Instruction::CLRF { reg } => {
            core.write(reg as u16, 0);
            core.status.z = true;
        }
        Instruction::CLRW => {
            core.w = 0;
            core.status.z = true;
        }
        Instruction::COMF { reg, dest } => {
            let result = !core.read(reg as u16);
            core.status.z = result == 0;
            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::DECF { reg, dest } => {
            let result = core.read(reg as u16) - 1;
            core.status.z = result == 0;
            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::DECFSZ { reg, dest } => {
            let result = core.read(reg as u16) - 1;
            if result == 0 {
                core.skip_next = true;
            }
            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::INCF { reg, dest } => {
            let result = core.read(reg as u16) + 1;
            core.status.z = result == 0;
            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::INCFSZ { reg, dest } => {
            let result = core.read(reg as u16) + 1;
            if result == 0 {
                core.skip_next = true;
            }
            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::IORWF { reg, dest } => {
            let result = core.w | core.read(reg as u16);
            core.status.z = result == 0;
            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::MOVF { reg, dest } => {
            let f = core.read(reg as u16);

            if dest {
                core.write(reg as u16, f);
            } else {
                core.w = f;
            }
        }
        Instruction::MOVWF { reg } => {
            core.write(reg as u16, core.w);
        }
        Instruction::NOP => {}
        Instruction::RLF { reg, dest } => {
            let v = core.read(reg as u16);

            let c = v >> 7;
            let result = (v << 1) | core.status.c as u8;
            core.status.c = c == 1;

            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = v;
            }
        }
        Instruction::RRF { reg, dest } => {
            let v = core.read(reg as u16);

            let c = v & 1;
            let result = (v >> 1) | ((core.status.c as u8) << 7);
            core.status.c = c == 1;

            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::SUBWF { reg, dest } => {
            let f = core.read(reg as u16);
            let (result, c) = f.overflowing_sub(core.w);

            core.status.z = result == 0;
            core.status.c = c;
            core.status.dc = (f & 0x0F) >= (core.w & 0x0F);

            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::SWAPF { reg, dest } => {
            let f = core.read(reg as u16);
            let result = ((f & 0xF0) >> 4) | ((f & 0xF) << 4);

            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::XORWF { reg, dest } => {
            let f = core.read(reg as u16);
            let result = core.w ^ f;

            core.status.z = result == 0;

            if dest {
                core.write(reg as u16, result);
            } else {
                core.w = result;
            }
        }
        Instruction::BCF { reg, bit } => {
            let result = core.read(reg as u16) & !(1u8 << bit);
            core.write(reg as u16, result);
        }
        Instruction::BSF { reg, bit } => {
            let result = core.read(reg as u16) | (1u8 << bit);
            core.write(reg as u16, result);
        }
        Instruction::BTFSC { reg, bit } => {
            let bit = core.read(reg as u16) & (1u8 << bit) == 1;
            if !bit {
                core.skip_next = true;
            }
        }
        Instruction::BTFSS { reg, bit } => {
            let bit = core.read(reg as u16) & (1u8 << bit) == 1;
            if bit {
                core.skip_next = true;
            }
        }
        Instruction::ADDLW { lit } => {
            let (result, c) = core.w.overflowing_add(lit);

            core.status.z = result == 0;
            core.status.c = c;
            core.status.dc = ((core.w & 0x0F) + (lit & 0x0F)) > 0x0F;

            core.w = result;
        }
        Instruction::ANDLW { lit } => {
            core.w &= lit;
            core.status.z = core.w == 0;
        }
        Instruction::CALL { lit } => {
            core.stack.push_front(core.pc);
            core.pc = (core.pclath as u16 & 0x18 << 7) | lit & 0x3FF;
        }
        Instruction::GOTO { lit } => {
            core.pc = (core.pclath as u16 & 0x18 << 7) | lit & 0x3FF;
        }
        Instruction::IORLW { lit } => {
            core.w |= lit;
            core.status.z = core.w == 0;
        }
        Instruction::MOVLW { lit } => {
            core.w = lit;
        }
        Instruction::RETFIE => {
            core.pc = core.stack.pop_front().unwrap();
            core.intcon.gie = true;
        }
        Instruction::RETLW { lit } => {
            core.w = lit;
            core.pc = core.stack.pop_front().unwrap();
        }
        Instruction::RETURN => {
            core.pc = core.stack.pop_front().unwrap();
        }
        Instruction::SUBLW { lit } => {
            let (result, c) = lit.overflowing_sub(core.w);

            core.status.z = result == 0;
            core.status.c = c;
            core.status.dc = (lit & 0x0F) >= (core.w & 0x0F);

            core.w = result;
        }
        Instruction::XORLW { lit } => {
            core.w ^= lit;
            core.status.z = core.w == 0;
        }
    }
}
