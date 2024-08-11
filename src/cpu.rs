pub struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            program_counter: 0,
            memory: [0; 0x1000],
            stack: [0; 16],
            stack_pointer: 0,
        }
    }
    pub fn write_opcode(&mut self, opcode: u16) {
        //self.current_operation = opcode;
        let p = self.program_counter;
        let op_byte1 = ((opcode & 0xFF00) >> 8) as u8;
        let op_byte2 = (opcode & 0x00FF) as u8;
        self.memory[p] = op_byte1;
        self.memory[p + 1] = op_byte2;
    }
    pub fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let mut op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1];
        op_byte1 = ((op_byte1 as u16) << 8) | (op_byte2 as u16);
        op_byte1
    }
    pub fn set_program_counter(&mut self, counter: u16) -> Result<(), String> {
        if (counter > (self.memory.len() as u16)) || ((counter % 2) != 0) {
            Err("cannot set program counter".to_string())
        } else {
            self.program_counter = counter as usize;
            Ok(())
        }
    }
    pub fn write_register(&mut self, num: usize, value: u8) -> Result<(), String> {
        if num > self.registers.len() {
            Err("cannot write".to_string())
        } else {
            self.registers[num] = value;
            Ok(())
        }
    }
    pub fn read_register(&self, num: usize) -> u8 {
        self.registers[num]
    }
    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            let nnn = opcode & 0x0FFF;
            match (c, x, y, d) {
                (0x0, 0x0, 0x0, 0x0) => return,
                (0x0, 0x0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (_, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] += self.registers[y as usize];
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
    fn call(&mut self, addr: u16) {
        if self.stack_pointer > self.stack.len() {
            panic!("Stack overflow!")
        }
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!")
        }
        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cmp_opcode() {
        let mut cpu = CPU::new();
        let op_code = 0x8014;
        cpu.write_opcode(op_code);
        assert_eq!(cpu.read_opcode(), op_code);
    }
    #[test]
    fn cmp_write_register() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.write_register(0, 10), Ok(()));
        assert_eq!(cpu.write_register(1, 10), Ok(()));
        assert_eq!(cpu.write_register(2, 10), Ok(()));
    }
    #[test]
    fn cmp_read_register() {
        let mut cpu = CPU::new();
        cpu.write_register(0, 255).unwrap();
        cpu.write_register(1, 0).unwrap();
        assert_eq!(cpu.read_register(0), 255);
        assert_ne!(cpu.read_register(1), 255);
    }
    #[test]
    fn cmp_add_xy() {
        let mut cpu = CPU::new();
        cpu.write_register(0, 10).unwrap();
        cpu.write_register(1, 20).unwrap();
        cpu.add_xy(0, 1);
        assert_eq!(cpu.read_register(0), 30);
    }
    #[test]
    fn cmp_read_write_opcode() {
        let mut cpu = CPU::new();
        let write_code = 0x1234;
        cpu.write_opcode(write_code);
        let read_code = cpu.read_opcode();
        assert_eq!(write_code, read_code);
    }
    #[test]
    fn cmp_stack() {
        let mut cpu = CPU::new();
        let program_counter = 0x100 as u16;
        cpu.call(program_counter);
        assert_eq!(cpu.program_counter, program_counter as usize);
        cpu.ret();
        assert_eq!(cpu.program_counter, 0x000 as usize);
    }
    #[test]
    fn cmp_setcounter() {
        let mut cpu = CPU::new();
        let pc0 = 0x100 as u16;
        let pc1 = 0x101 as u16;
        let pc2 = 0x1000 as u16;
        let pc3 = 0x0FFF as u16;
        let pc4 = 0x0FFE as u16;
        cpu.set_program_counter(pc0).unwrap();
        assert_eq!(cpu.program_counter, pc0 as usize);

        assert_eq!(
            Err("cannot set program counter".to_string()),
            cpu.set_program_counter(pc1)
        );
        cpu.set_program_counter(pc2).unwrap();
        assert_eq!(cpu.program_counter, pc2 as usize);

        assert_eq!(
            Err("cannot set program counter".to_string()),
            cpu.set_program_counter(pc3)
        );

        cpu.set_program_counter(pc4).unwrap();
        assert_eq!(cpu.program_counter, pc4 as usize);
    }
}
