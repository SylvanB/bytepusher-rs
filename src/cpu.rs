use crate::Memory;

#[derive(Copy, Clone)]
pub struct Cpu {
    mem: Memory,
    pc: usize,
}

impl Cpu {
    pub fn init() -> Self {
        let mem = Memory::init();
        Self {
            mem,
            pc: 2,
        }
    }

    pub fn calculate_frame_data(mut self) {
        for _ in 0..65536 {
            self.exec_instruction();
        }
    }

    fn exec_instruction(&mut self) {
        let addr_a = self.pc;

        self.pc += 1;
        let addr_b = self.pc;
        self.mem.copy_value(addr_b, addr_a);

        self.pc += 1;
        let jmp_loc = self.pc;
        self.pc = jmp_loc;
    }
}
