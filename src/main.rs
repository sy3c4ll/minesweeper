use iced::{Sandbox, Settings};
use minesweeper::gui::Gui;

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
