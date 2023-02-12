use std::path::{PathBuf, Path};

use iced::widget::scrollable::Properties;
use iced::widget::{button, column, row, text, scrollable, slider, radio, container, progress_bar, vertical_space, horizontal_space, image};
use iced::{Settings, Alignment, Application, Theme, executor, Command, Length, Element, Color, theme, Renderer};
use once_cell::sync::Lazy;
use tile_pane::ScrollMessage;

mod grid;
mod tile_pane;
use crate::grid::Grid;
use crate::tile_pane::ImageTile;


static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

fn main() -> iced::Result {
    Demo::run(Settings::default())
}


struct Demo {
    tile_size: u16,
    current_scroll_offset: scrollable::RelativeOffset,
    images: Vec<ImageTile>,
}



impl Application for Demo {
    type Executor = executor::Default;
    type Message = ScrollMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let images = ["resources/still_1.jpeg", "resources/still_2.png", "resources/still_3.webp"];
        let images = images.iter().enumerate().map(|(i, &p)| ImageTile::load(i as u32, p)).collect();

        (
            Self {
                tile_size: 256,
                current_scroll_offset: scrollable::RelativeOffset::START,
                images
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Counter - Iced".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::ScrollToBeginning => {
                self.current_scroll_offset = scrollable::RelativeOffset::START;
                scrollable::snap_to(SCROLLABLE_ID.clone(), self.current_scroll_offset)
            },
            Self::Message::Scrolled(offset) => {
                self.current_scroll_offset = offset;
                Command::none()
            },
            Self::Message::ZoomChanged(zoom) => {
                self.tile_size = zoom;
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let zoom_slider = slider(50..=512, self.tile_size, Self::Message::ZoomChanged);

        let scroll_to_beginning = || { button("Scroll to beginning").padding(10).on_press(Self::Message::ScrollToBeginning) };

        let scrollable_content: Element<Self::Message> = Element::from(scrollable(
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
            .vertical_scroll(scrollbar_properties())
            .id(SCROLLABLE_ID.clone())
            .on_scroll(Self::Message::Scrolled),
        );

        let content: Element<Self::Message> = column!(scrollable_content, zoom_slider).spacing(10).into();

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

fn scrollbar_properties() -> Properties {
    Properties::new()
        .width(10)
        .margin(0)
        .scroller_width(10)
}
