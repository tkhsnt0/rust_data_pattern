mod cpu;
//mod endian;
use cpu::CPU;
//use endian::{BigEndianU32, LittleEndianU32};
fn main() {
    let mut cpu = CPU::new();
    // register設定
    cpu.write_register(0, 5).unwrap();
    cpu.write_register(1, 10).unwrap();
    // pcとopcode設定
    cpu.set_program_counter(0x000).unwrap();
    cpu.write_opcode(0x2100);
    cpu.set_program_counter(0x002).unwrap();
    cpu.write_opcode(0x2100);
    cpu.set_program_counter(0x004).unwrap();
    cpu.write_opcode(0x0000);
    cpu.set_program_counter(0x100).unwrap();
    cpu.write_opcode(0x8014);
    cpu.set_program_counter(0x102).unwrap();
    cpu.write_opcode(0x8014);
    cpu.set_program_counter(0x104).unwrap();
    cpu.write_opcode(0x00EE);
    //pcを0x000に設定して開始
    cpu.set_program_counter(0x000).unwrap();
    cpu.run();
    assert_eq!(cpu.read_register(0), 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.read_register(0));
}
