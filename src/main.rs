use iced::{Settings, Element, widget::{Column, Text, Container}, Length, alignment::{Horizontal, Vertical}, Theme, Application, Command};
use iced_aw::{TabBarPosition, Tabs, TabLabel};

mod grid;
mod tile_pane;
mod todo;
mod checklist;
mod nested_list;
// mod selection_tree;
// mod trees;
mod lazy_scroll;

use lazy_scroll::LazyScroll;
use nested_list::NestedListTab;
use tile_pane::TilePane;

fn main() -> iced::Result {
    // tile_pane::TilePaneDemo::run(Settings::default())
    // todo::Todos::run(Settings::default())
    // checklist::Checklist::run(Settings::default())
    // nested_list::NestedList::run(Settings::default())
    // selection_tree::SelectionTree::run(Settings::default())
    Example::run(Settings::default())
}


#[derive(Clone, Debug)]
enum Message {
    TabSelected(usize), // TODO: Make enum
    TilePane(tile_pane::Message),
    NestedList(nested_list::Message),
    LazyScroll(lazy_scroll::Message)
}

struct Example {
    active_tab: usize,
    tile_tab: TilePane,
    list_tab: NestedListTab,
    lazy_scroll: LazyScroll,
}

impl Application for Example {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                active_tab: 0,
                tile_tab: TilePane::new(),
                list_tab: NestedListTab::new(),
                lazy_scroll: LazyScroll {  }
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Example".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::TabSelected(selected) => {
                self.active_tab = selected;
                Command::none()
            },
            Self::Message::TilePane(message) => { self.tile_tab.update(message).map(|msg| Self::Message::TilePane(msg)) },
            Self::Message::NestedList(message) => { self.list_tab.update(message).map(|msg| Self::Message::NestedList(msg)) },
            Self::Message::LazyScroll(_) => todo!(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {        
        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.tile_tab.tab_label(), self.tile_tab.view().map(|msg| Self::Message::TilePane(msg)))
            .push(self.list_tab.tab_label(), self.list_tab.view().map(|msg| Self::Message::NestedList(msg)))
            .tab_bar_position(TabBarPosition::Top)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }
}


enum Icon {
    Tile,
    List,
    // Calc,
    // CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Tile => '\u{E800}',
            Icon::List => '\u{E801}',
            // Icon::Calc => '\u{F1EC}',
            // Icon::CogAlt => '\u{E802}',
        }
    }
}

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}

