use iced::{Settings, Application};

mod grid;
mod tile_pane;
mod todo;
mod checklist;
fn main() -> iced::Result {
    // tile_pane::TilePaneDemo::run(Settings::default())
    // todo::Todos::run(Settings::default())
    checklist::Checklist::run(Settings::default())
}

