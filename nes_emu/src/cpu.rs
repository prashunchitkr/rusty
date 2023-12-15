use crate::{
    opcode,
    utils::consts::{flags, AddressingMode},
};

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&self, addr: u16) -> u16;
    fn mem_write_u16(&mut self, addr: u16, data: u16);
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 0x01) as u16;

        (hi << 0x08) | lo
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let lo = (data & 0xFF) as u8;
        let hi = (data >> 0x08) as u8;

        self.mem_write(addr, lo);
        self.mem_write(addr + 0x01, hi);
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0xFD,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::ZeroPage_X => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_x) as u16,

            AddressingMode::ZeroPage_Y => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_y) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::Absolute_X => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_x as u16),

            AddressingMode::Absolute_Y => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_y as u16),

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(0x01) as u16);

                (hi as u16) << 8 | (lo as u16)
            }

            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(0x01) as u16);

                let ptr = (hi as u16) << 8 | (lo as u16);

                ptr.wrapping_add(self.register_y as u16)
            }

            AddressingMode::NoneAddressing => panic!("Addressing mode {:?} not supported", mode),
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.stack_pointer = 0xFD;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(0x01);
        self.update_flags(self.register_x);
    }

    fn update_flags(&mut self, register: u8) {
        if register == 0x00 {
            self.status |= flags::ZERO;
        } else {
            self.status &= 0b1111_1101;
        }

        if register & flags::NEGATIVE != 0x00 {
            self.status |= flags::NEGATIVE;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    pub fn run(&mut self) {
        let ref opcodes = *opcode::OPCODES_MAP;

        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter = self.program_counter.wrapping_add(0x01);
            let program_counter_state = self.program_counter;

            let opcode = opcodes
                .get(&code)
                .expect(&format!("Unrecognized opcode {:x}", code));

            match code {
                0x00 => break,

                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.addressing_mode)
                }

                0xAA => self.tax(),

                0xE8 => self.inx(),

                _ => panic!("Unrecognized opcode {:x}", code),
            }

            if program_counter_state == self.program_counter {
                self.program_counter = self.program_counter.wrapping_add((opcode.size - 1) as u16);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_new() {
        let cpu = CPU::new();

        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.stack_pointer, 0xFD);
        assert_eq!(cpu.status, 0);
        assert_eq!(cpu.program_counter, 0);
    }

    #[test]
    fn test_0x00_brk() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0x00]);

        assert_eq!(cpu.program_counter, 0x8001);
    }

    #[test]
    fn test_0xa9_lda() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xA9, 0x05, 0x00]);

        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status & flags::ZERO, 0x00);
        assert_eq!(cpu.status & flags::NEGATIVE, 0x00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status & flags::ZERO, flags::ZERO);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xA9, 0xFF, 0x00]);

        assert_eq!(cpu.register_a, 0xFF);
        assert_eq!(cpu.status & flags::NEGATIVE, flags::NEGATIVE);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xA9, 0x05, 0xAA, 0x00]);

        assert_eq!(cpu.register_x, 0x05);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_0xe8_inx() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xE8, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0x02)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_x = 0xFF;
        cpu.load(vec![0xE8, 0xE8, 0x00]);

        cpu.run();

        assert_eq!(cpu.register_x, 0x01)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xA5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }
}
