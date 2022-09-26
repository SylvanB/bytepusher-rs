#[derive(Copy, Clone)]
pub struct Memory {
    mem: [u8; 0x100000]
}

impl Memory {
    pub fn init() -> Self {
        Self {
            mem: [0; 0x100000]
        }
    }

    pub fn copy_value(mut self, dst: usize, src: usize) {
        self.mem[dst] = self.mem[src];
    }

    fn addr_at(&self, dst: usize) -> usize {
        (self.mem[dst] as usize) << 16 | (self.mem[dst + 1] as usize) << 8 | self.mem[dst + 2] as usize
    }

    fn byte_at(&self, dst: usize) -> u8 {
        self.mem[dst]
    }
}
