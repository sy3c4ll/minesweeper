use iced::{Element, Length, Sandbox};
use iced::widget::{button, text, Column, Row};
use super::board::{Board, State};
use super::cell::Visibility;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Gui {
    pub board: Board::<{ Self::WIDTH }, { Self::HEIGHT }>,
}

impl Gui {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const MINES: usize = 10;
}

impl Sandbox for Gui {
    type Message = Message;
    fn new() -> Self {
        Self {
            board: Board::<{ Self::WIDTH }, { Self::HEIGHT }>::new(Self::MINES),
        }
    }
    fn title(&self) -> String { "Minesweeper".to_owned() }
    fn update(&mut self, message: Self::Message) {
        if self.board.state() == State::InProgress {
            let _ = match message {
                Message::Flag(x, y) => self.board.flag(x, y),
                Message::Unflag(x, y) => self.board.unflag(x, y),
                Message::Reveal(x, y) => self.board.reveal(x, y),
            };
        } else {
            std::process::exit(0);
        }
    }
    fn view(&self) -> Element<Self::Message> {
        Row::with_children(
            self.board.board.iter().enumerate()
                .map(|(x, col)| Column::with_children(
                    col.iter().enumerate()
                        .map(|(y, cell)| match cell.visibility {
                            Visibility::Hidden => button("").on_press(Message::Reveal(x, y)),
                            Visibility::Visible => match cell.is_mine {
                                true => button("M"),
                                false => button(text(cell.neighbouring_mines)),
                            },
                            Visibility::Flagged => button("F").on_press(Message::Reveal(x, y)),
                        }.width(Length::Fill).height(Length::Fill).into())
                ).width(Length::Fill).height(Length::Fill).into())
        ).width(Length::Fill).height(Length::Fill).into()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Message {
    Flag(usize, usize),
    Unflag(usize, usize),
    Reveal(usize, usize),
}
