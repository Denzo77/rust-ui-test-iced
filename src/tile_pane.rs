use std::path::PathBuf;
use once_cell::sync::Lazy;

use iced::{widget::{scrollable::RelativeOffset, image, slider, button, column, container}, Length, Element, Alignment, Application, executor, Theme, Command};
use iced::widget::scrollable;


use crate::grid::Grid;

pub struct TilePaneDemo {
    tile_pane: TilePane,
}

impl Application for TilePaneDemo {
    type Executor = executor::Default;
    type Message = ScrollMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let images = ["resources/still_1.jpeg", "resources/still_2.png", "resources/still_3.webp"];
        let images = images.iter().enumerate().map(|(i, &p)| ImageTile::load(i as u32, p)).collect();

        
        (
            Self {
                tile_pane: TilePane::from_images(images)
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Counter - Iced".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self.tile_pane.update(message) {
            ScrollCommand::None => Command::none(),
            ScrollCommand::ScrollToStart { id, offset } => scrollable::snap_to(id, offset),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = self.tile_pane.view();

        container(content)
            .width(Length::Fill).height(Length::Fill)
            .padding(40)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}



const DEFAULT_TILE_SIZE: u16 = 128;
static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

#[derive(Debug, Clone, Copy)]
pub enum ScrollMessage {
    ScrollToStart,
    Scrolled(scrollable::RelativeOffset),
    ZoomChanged(u16),
}

#[derive(Debug, Clone)]
pub enum ScrollCommand {
    None,
    ScrollToStart { id: scrollable::Id, offset: RelativeOffset }
}

pub struct TilePane {
    tile_size: u16,
    scroll_offset: scrollable::RelativeOffset,
    images: Vec<ImageTile>,
}

impl TilePane {
    pub fn new() -> Self {
        Self::from_images(Vec::new())
    }

    pub fn from_images(images: Vec<ImageTile>) -> Self {
        Self {
            tile_size: DEFAULT_TILE_SIZE,
            scroll_offset: scrollable::RelativeOffset::START,
            images,
        }
    }

    pub fn update(&mut self, message: ScrollMessage) -> ScrollCommand {
        match message {
            ScrollMessage::ScrollToStart => {
                self.scroll_offset = scrollable::RelativeOffset::START;
                ScrollCommand::ScrollToStart { id: SCROLLABLE_ID.clone(), offset: self.scroll_offset }
            },
            ScrollMessage::Scrolled(offset) => {
                self.scroll_offset = offset;
                ScrollCommand::None
            },
            ScrollMessage::ZoomChanged(zoom) => {
                self.tile_size = zoom;
                ScrollCommand::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, ScrollMessage> {
        let zoom_slider = slider(50..=512, self.tile_size, ScrollMessage::ZoomChanged);

        let scroll_to_beginning = || { button("Scroll to beginning").padding(10).on_press(ScrollMessage::ScrollToStart) };

        let scrollable_content: Element<ScrollMessage> = Element::from(scrollable(
                column!(
                    Grid::with_children(self.images.iter()
                        .map(|img| img.view(Length::Units(self.tile_size))).collect())
                        .column_width(self.tile_size),
                    scroll_to_beginning()
                )
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .padding([40, 0, 40, 0])
                .spacing(40)
            )
            .height(Length::Fill)
            .vertical_scroll(theming::scrollbar_properties())
            .id(SCROLLABLE_ID.clone())
            .on_scroll(ScrollMessage::Scrolled),
        );

        column!(scrollable_content, zoom_slider).spacing(10).into()
    }
}



pub struct ImageTile {
    uid: u32,
    path: PathBuf,
    handle: image::Handle,
}

impl ImageTile {
    pub fn load(uid: u32, path: &str) -> Self {
        Self {
            uid,
            path: path.into(),
            handle: image::Handle::from_path(path),
        }
    }

    pub fn view(&self, size: Length) -> Element<ScrollMessage> {
        image::Image::new(self.handle.clone())
            .width(size)
            .height(size)
            .into()
    }
}

mod theming {
    use iced::widget::scrollable::Properties;

    pub fn scrollbar_properties() -> Properties {
        Properties::new()
            .width(10)
            .margin(0)
            .scroller_width(10)
    }    
}