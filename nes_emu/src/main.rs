mod cpu;
mod opcode;
mod utils;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
}
