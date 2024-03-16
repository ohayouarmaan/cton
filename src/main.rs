use crate::memory::Memory;
#[macro_use] extern crate prettytable;
use crate::cpu::Cpu;

mod cpu;
mod memory;
fn main() {
    let mut m1 = Memory::new(4);
    m1.set_data(0, 0b00000010_00000100);
    m1.set_data(1, 0b00000110_00010000);
    m1.set_data(2, 0b00000010_00001000);
    m1.set_data(3, 0b00011010_00000100);
    let mut c1 = Cpu::new(Some(m1));
    c1._loop();
}
