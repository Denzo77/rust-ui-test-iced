use iced::{Settings, Application};

mod grid;
mod tile_pane;
mod todo;

fn main() -> iced::Result {
    // tile_pane::TilePaneDemo::run(Settings::default())
    todo::Todos::run(Settings::default())
}

