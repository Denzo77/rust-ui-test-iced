use iced::{Settings, Application};

mod grid;
mod tile_pane;
mod todo;
mod checklist;
mod nested_list;
mod selection_tree;
mod trees;

fn main() -> iced::Result {
    // tile_pane::TilePaneDemo::run(Settings::default())
    // todo::Todos::run(Settings::default())
    // checklist::Checklist::run(Settings::default())
    // nested_list::NestedList::run(Settings::default())
    selection_tree::SelectionTree::run(Settings::default())
}

