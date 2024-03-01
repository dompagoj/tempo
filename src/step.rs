pub struct Step {
    pos: u32,
    len: u32,
}

impl Step {
    pub fn new(len: u32) -> Self {
        Self { pos: 1, len }
    }

    pub fn get_pos(&self) -> u32 {
        self.pos
    }

    pub fn inc_pos(&mut self) {
        self.pos += 1;
    }

    pub fn get_str(&mut self) -> String {
        assert!(self.get_pos() <= self.len);
        let str = format!("[{}/{}]", self.pos, self.len);
        self.inc_pos();

        return str;
    }
}
