use crate::Memory;

pub struct Cpu<'cpu> {
    mem: &'cpu mut Memory,
    pc: usize,
}

impl<'cpu> Cpu<'cpu> {
    pub fn init(mem: &'cpu mut Memory) -> Self {
        Self {
            mem,
            pc: 2,
        }
    }

    pub fn calculate_frame_data(&mut self) {
        for _ in 0..65536 {
            self.exec_instruction();
            self.mem.write_addr_at(0x3, self.pc);
        }
    }

    pub fn get_pixeldata_block(&self, offset: usize) -> &[u8] {
        self.mem.get_pixeldata_block(0x5)
    }

    fn get_next_address(&mut self) -> usize {
        let curr = self.mem.addr_at(self.pc);
        self.pc += 3;

        curr
    }

    fn exec_instruction(&mut self) {
        // let addr_a = self.pc;
        let addr_a = self.get_next_address();

        // let addr_b = self.pc;
        let addr_b = self.get_next_address();
        self.mem.copy_value(addr_b, addr_a);

        // let jmp_loc = self.mem.byte_at(self.pc) as usize;
        let jmp_loc = self.get_next_address();
        self.pc = jmp_loc;
    }
}
