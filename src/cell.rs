#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Cell {
    pub is_mine: bool,
    pub visibility: Visibility,
    pub neighbouring_mines: u8,
}

impl Cell {
    pub fn new(is_mine: bool) -> Self {
        Self {
            is_mine,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Visibility {
    #[default] Hidden,
    Visible,
    Flagged,
}
