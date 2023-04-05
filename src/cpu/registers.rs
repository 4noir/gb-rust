use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Flags : u8 {
        const Z = 0b10000000;
        const N = 0b01000000;
        const H = 0b00100000;
        const C = 0b00010000;
    }
}

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub struct SplitReg {
    pub lo: u8,
    pub hi: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SplitRegFlag {
    pub lo: Flags,
    pub hi: u8,
}

#[repr(C)]
pub union Register {
    pub full: u16,
    pub split: SplitReg,
}

#[repr(C)]
pub union RegisterFlag {
    pub full: u16,
    pub split: SplitRegFlag,
}

impl RegisterFlag {
    pub fn new() -> Self {
        RegisterFlag { full: 0 }
    }
}

impl Register {
    pub fn new() -> Self {
        Register { full: 0 }
    }
}

#[cfg(test)]
mod reg_tests {
    use super::*;
    use bitflags::BitFlags;
    #[test]
    pub fn reg_test_zero() {
        let expected = Register {
            split: SplitReg { hi: 0x0, lo: 0x0 },
        };
        let actual = Register { full: 0x0 };
        unsafe {
            assert_eq!(expected.full, actual.full);
            assert_eq!(expected.split.hi, actual.split.hi);
            assert_eq!(expected.split.lo, actual.split.lo);
        }
    }

    #[test]
    pub fn reg_test_full() {
        let expected = Register {
            split: SplitReg { hi: 0xFF, lo: 0xFF },
        };
        let actual = Register { full: 0xFFFF };
        unsafe {
            assert_eq!(expected.full, actual.full);
            assert_eq!(expected.split.hi, actual.split.hi);
            assert_eq!(expected.split.lo, actual.split.lo);
        }
    }

    #[test]
    pub fn reg_test_abcd() {
        let expected = Register {
            split: SplitReg { hi: 0xAB, lo: 0xCD },
        };
        let actual = Register { full: 0xABCD };
        unsafe {
            assert_eq!(expected.full, actual.full);
            assert_eq!(expected.split.hi, actual.split.hi);
            assert_eq!(expected.split.lo, actual.split.lo);
        }
    }

    #[test]
    pub fn test_flag_all() {
        let expected = RegisterFlag {
            split: SplitRegFlag {
                hi: 0x00,
                lo: Flags::all(),
            },
        };
        let actual = RegisterFlag { full: 0x00F0 };
        unsafe {
            assert_eq!(expected.split.lo.bits(), actual.split.lo.bits());
        }
    }
    #[test]
    pub fn test_flag_zero() {
        let expected = RegisterFlag {
            split: SplitRegFlag {
                hi: 0x00,
                lo: Flags::empty(),
            },
        };
        let actual = RegisterFlag { full: 0x0000 };
        unsafe {
            assert_eq!(expected.split.lo.bits(), actual.split.lo.bits());
        }
    }
    #[test]
    pub fn test_flag() {
        let expected = RegisterFlag {
            split: SplitRegFlag {
                hi: 0xAB,
                lo: Flags::H,
            },
        };
        let actual = RegisterFlag { full: 0xAB20 };
        unsafe {
            assert_eq!(expected.split.lo.bits(), actual.split.lo.bits());
        }
    }

    #[test]
    pub fn test_union_size() {
        assert_eq!(std::mem::size_of::<Flags>(), 1);
    }
}
