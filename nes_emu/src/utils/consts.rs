#[allow(dead_code)]
pub mod flags {
    pub const CARRY: u8 = 0b0000_0001;
    pub const ZERO: u8 = 0b0000_0010;
    pub const INTERRUPT_DISABLE: u8 = 0b0000_0100;
    pub const DECIMAL_MODE: u8 = 0b0000_1000;
    pub const BREAK: u8 = 0b0001_0000;
    pub const BREAK2: u8 = 0b0010_0000;
    pub const OVERFLOW: u8 = 0b0100_0000;
    pub const NEGATIVE: u8 = 0b1000_0000;
}

#[derive(Debug)]
#[allow(non_camel_case_types, dead_code)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}
