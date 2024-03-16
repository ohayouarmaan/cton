pub struct Memory {
    data: Vec<u16>,
    pub len: usize
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let mut data: Vec<u16> = Vec::new();
        for _ in 0..size {
            data.push(0b0000000000000000);
        }
        Memory {
            data,
            len: size
        }
    }
    pub fn get_data(&self, index: usize) -> Result<u16, &str> {
        if index >= 0 && index < self.data.len() {
            return Ok(self.data[index]);
        }
        return Err("[MEMORY]: No such memory location exists.");
    }
    
    pub fn set_data(&mut self, index: usize, data: u16) -> bool {
        if index < self.data.len() {
            self.data[index] = data;
            return true;
        }
        return false;
    }
}
