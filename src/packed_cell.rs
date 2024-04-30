#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackedCell (u8);

impl PackedCell {
    pub fn new(is_mine: bool) -> Self {
        if is_mine {
            Self(0b00010000)
        } else {
            Self(0b00000000)
        }
    }
    pub fn new_full(is_mine: bool, is_visible: bool, is_marked: bool, neighbouring_mines: u8) -> Self {
        Self((neighbouring_mines & 0b1111) | ((is_mine as u8) >> 4) | ((is_visible as u8) >> 5) | ((is_marked as u8) >> 6))
    }
    pub fn is_mine(&self) -> bool {
        ((self.0 >> 4) & 0b1) != 0
    }
    pub fn set_mine(&mut self, is_mine: bool) {
        self.0 = (self.0 | (1 << 4)) & ((is_mine as u8) >> 4);
    }
    pub fn is_visible(&self) -> bool {
        ((self.0 >> 5) & 0b1) != 0
    }
    pub fn set_visible(&mut self, is_visible: bool) {
        self.0 = (self.0 | (1 << 5)) & ((is_visible as u8) >> 5);
    }
    pub fn is_marked(&self) -> bool {
        ((self.0 >> 6) & 0b1) != 0
    }
    pub fn set_marked(&mut self, is_marked: bool) {
        self.0 = (self.0 | (1 << 6)) & ((is_marked as u8) >> 6);
    }
    pub fn neighbouring_mines(&self) -> u8 {
        self.0 & 0b1111
    }
    pub fn set_neighbouring_mines(&mut self, neighbouring_mines: u8) {
        self.0 = (self.0 | 0b1111) & (neighbouring_mines & 0b1111);
    }
    pub fn as_raw(&self) -> &u8 { &self.0 }
    pub fn as_raw_mut(&mut self) -> &mut u8 { &mut self.0 }
}
