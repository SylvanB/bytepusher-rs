pub struct Memory {
    mem: [u8; 0x100000]
}

impl Memory {
    pub fn init() -> Self {
        Self {
            mem: [0; 0x100000]
        }
    }

    pub fn init_from_disk_img(img: Vec<u8>) -> Self {
        let mut mem = [0; 0x100000];
        let mut img = img;
        img.resize(mem.len(), 0);
        mem.copy_from_slice(&img);

        Self {
            mem
        }
    }

    pub fn copy_value(&mut self, dst: usize, src: usize) {
        self.mem[dst] = self.mem[src];
    }

    pub fn write_byte_at(&mut self, dst: usize, val: u8) {
        self.mem[dst] = val;
    }

    pub fn write_addr_at(&mut self, dst: usize, val: usize) {
        let a: u8 = (val & 0xff0000 >> 16) as u8;
        let b: u8 = (val & 0xff00 >> 8) as u8;
        let c: u8 = (val & 0xff) as u8;
        self.write_byte_at(dst, a);
        self.write_byte_at(dst, b);
        self.write_byte_at(dst, c);
    }

    pub fn addr_at(&self, dst: usize) -> usize {
        (self.mem[dst] as usize) << 16 | (self.mem[dst + 1] as usize) << 8 | self.mem[dst + 2] as usize
    }

    pub fn byte_at(&self, dst: usize) -> u8 {
        self.mem[dst]
    }

    pub fn get_pixeldata_block(&self, offset: usize) -> &[u8] {
        &self.mem[offset..(offset + 0x10000)]
    }
}
