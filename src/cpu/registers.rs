use bitflags::bitflags;
use paste::paste;

macro_rules! make_reg_funcs {
    ($hi:ident, $lo:ident) => {
        paste! {
            impl Registers {
                pub fn [< $hi $lo >](&self) -> u16 {
                    return ((self.[< $hi >] as u16) << 8) | (self.[< $lo >] as u16);
                }
                pub fn [< set_ $hi $lo >](&mut self, value : u16) {
                    self.[< $hi >] = (value >> 8) as u8;
                    self.[< $lo >] = (value & 0x00FF) as u8;
                }
            }
        }
    };
}

bitflags! {
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub struct Flags : u8 {
        const Z = 0b10000000;
        const N = 0b01000000;
        const H = 0b00100000;
        const C = 0b00010000;
        const as_num = Self::Z.bits() | Self::N.bits() | Self::H.bits() | Self::C.bits();
    }
}

#[derive(PartialEq, Debug)]
pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn af(&self) -> u16 {
        return ((self.a as u16) << 8) | (self.f.bits() as u16 & 0x00F0);
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f =
            Flags::from_bits((value & 0x00F0) as u8).expect("incorrect flags in value for set_af");
    }

    pub fn new() -> Self {
        return Registers {
            a: 0,
            f: Flags::empty(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        };
    }
}

make_reg_funcs!(b, c);
make_reg_funcs!(d, e);
make_reg_funcs!(h, l);

#[cfg(test)]
mod reg_tests {
    use super::*;

    #[test]
    fn test_abcd() {
        let mut expected = Registers::new();
        expected.b = 0xAB;
        expected.c = 0xCD;
        let mut actual = Registers::new();
        actual.set_bc(0xABCD);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_abcd_flags() {
        let mut expected = Registers::new();
        expected.a = 0xAB;
        expected.f = Flags::as_num;
        let mut actual = Registers::new();
        actual.set_af(0xABF0);
        assert_eq!(expected, actual);
    }
}
