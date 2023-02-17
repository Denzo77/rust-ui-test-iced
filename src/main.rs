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

use iced_native::row;
use lazy_scroll::LazyScroll;
use nested_list::TreeViewPane;
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
    list_tab: TreeViewPane,
    lazy_scroll: LazyScroll,
}

impl Application for Example {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                active_tab: 0,
                tile_tab: TilePane::new(),
                list_tab: TreeViewPane::new(),
                lazy_scroll: LazyScroll::new(),
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
            Self::Message::TilePane(message) => { self.tile_tab.update(message).map(Self::Message::TilePane) },
            Self::Message::NestedList(message) => { self.list_tab.update(message).map(Self::Message::NestedList) },
            Self::Message::LazyScroll(message) => { self.lazy_scroll.update(message).map(Self::Message::LazyScroll) },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {   
        let tabs = Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.tile_tab.tab_label(), self.tile_tab.view().map(Self::Message::TilePane))
            // .push(self.list_tab.tab_label(), self.list_tab.view().map(Self::Message::NestedList))
            .push(self.lazy_scroll.tab_label(), self.lazy_scroll.view().map(Self::Message::LazyScroll))
            .tab_bar_position(TabBarPosition::Top);
            // .into();

        row!(
            self.list_tab.content().map(Self::Message::NestedList),
            tabs
        ).into()
    }

    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }
}

enum Icon {
    _Tile,
    _List,
    // Calc,
    // CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::_Tile => '\u{E800}',
            Icon::_List => '\u{E801}',
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
