use iced::widget::{button, column, row, text, scrollable, slider, radio, container, progress_bar, vertical_space, horizontal_space, image};
use iced::{Settings, Alignment, Application, Theme, executor, Command, Length, Element, Color, theme, Renderer};
use tile_pane::{ScrollMessage, TilePane};

mod grid;
mod tile_pane;
use crate::tile_pane::ImageTile;

fn main() -> iced::Result {
    Demo::run(Settings::default())
}

struct Demo {
    tile_pane: TilePane,
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
                tile_pane: TilePane::from_images(images)
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Counter - Iced".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.tile_pane.update(message)
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
