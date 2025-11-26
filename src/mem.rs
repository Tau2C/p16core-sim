#[derive(Debug, Clone)]
pub struct Ram {
    data: [u8; 512],
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}

impl Ram {
    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn new() -> Self {
        Self { data: [0; 512] }
    }
}
