use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::index::sample;
use std::ops::{Index, IndexMut};
use super::cell::{Cell, Visibility};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Board<const W: usize, const H: usize> {
    pub board: [[Cell; H]; W],
    pub mines: usize,
    cleared: usize,
    state: State,
}

const D: &[(isize, isize)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];

impl<const W: usize, const H: usize> Board<W, H> {
    fn clear_plains(&mut self, x: usize, y: usize) {
        if self[x][y].neighbouring_mines == 0 {
            for (dx, dy) in D {
                let (cx, cy) = (((x as isize) + dx) as usize, ((y as isize) + dy) as usize);
                if cx < W && cy < H && self[cx][cy].visibility != Visibility::Visible {
                    self[cx][cy].visibility = Visibility::Visible;
                    self.cleared += 1;
                    if self[cx][cy].neighbouring_mines == 0 {
                        self.clear_plains(cx, cy);
                    }
                }
            }
        }
    }

    pub fn new(mines: usize) -> Self {
        let mut board = Board {
            mines,
            ..Default::default()
        };
        let mine_locations = sample(&mut SmallRng::from_entropy(), W * H, mines);
        for loc in mine_locations {
            let (lx, ly) = (loc % W, loc / W);
            board.board[lx][ly].is_mine = true;
            for (dx, dy) in D {
                let (cx, cy) = (((lx as isize) + dx) as usize, ((ly as isize) + dy) as usize);
                if cx < W && cy < H {
                    board.board[cx][cy].neighbouring_mines += 1;
                }
            }
        }
        board
    }
    pub fn new_with_clear(mines: usize, x: usize, y: usize) -> Self {
        let mut board = Board {
            mines,
            ..Default::default()
        };
        let mine_locations = sample(&mut SmallRng::from_entropy(), W * H - 1, mines);
        for loc in mine_locations {
            let (lx, ly) = match (loc % W, loc / W) {
                (lx, ly) if lx == x && ly == y => (W - 1, H - 1),
                (lx, ly) => (lx, ly),
            };
            board.board[lx][ly].is_mine = true;
            for (dx, dy) in D {
                let (cx, cy) = (((lx as isize) + dx) as usize, ((ly as isize) + dy) as usize);
                if cx < W && cy < H {
                    board.board[cx][cy].neighbouring_mines += 1;
                }
            }
        }
        board
    }
    pub fn flag(&mut self, x: usize, y: usize) -> Event {
        if self[x][y].visibility == Visibility::Hidden {
            self[x][y].visibility = Visibility::Flagged;
            Event::Continue
        } else {
            Event::InvalidOperation
        }
    }
    pub fn unflag(&mut self, x: usize, y: usize) -> Event {
        if self[x][y].visibility == Visibility::Flagged {
            self[x][y].visibility = Visibility::Hidden;
            Event::Continue
        } else {
            Event::InvalidOperation
        }
    }
    pub fn reveal(&mut self, x: usize, y: usize) -> Event {
        if self[x][y].visibility != Visibility::Visible {
            self[x][y].visibility = Visibility::Visible;
            self.cleared += 1;
            if self[x][y].is_mine {
                self.state = State::Defeat;
                Event::SteppedOnMine
            } else if self.cleared >= W * H - self.mines {
                self.state = State::Victory;
                Event::AllMinesCleared
            } else {
                if self[x][y].neighbouring_mines == 0 {
                    self.clear_plains(x, y);
                }
                Event::Continue
            }
        } else {
            Event::InvalidOperation
        }
    }
    pub fn state(&self) -> State { self.state }
}

impl<const W: usize, const H: usize> Default for Board<W, H> {
    fn default() -> Self {
        Self {
            board: [[Default::default(); H]; W],
            mines: Default::default(),
            cleared: Default::default(),
            state: Default::default(),
        }
    }
}

impl<const W: usize, const H: usize> Index<usize> for Board<W, H> {
    type Output = [Cell];
    fn index(&self, idx: usize) -> &Self::Output { &self.board[idx] }
}

impl<const W: usize, const H: usize> IndexMut<usize> for Board<W, H> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output { &mut self.board[idx] }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum State {
    #[default] InProgress,
    Victory,
    Defeat,
}

pub enum Event {
    Continue,
    InvalidOperation,
    AllMinesCleared,
    SteppedOnMine,
}
