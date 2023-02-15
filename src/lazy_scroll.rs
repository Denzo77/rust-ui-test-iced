use iced::{widget::{scrollable, container, text}, Command, Length, Element};
use iced_native::{widget::column, Widget};

use crate::Tab;

const DEFAULT_TILE_SIZE: u16 = 128;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Scrolled(scrollable::RelativeOffset),
    Size(u16, u16),
}

pub struct LazyScroll {
    elements: Vec<String>,
    current_offset: scrollable::RelativeOffset,
    width: u16,
    height: u16,
}

impl LazyScroll {
    pub fn new() -> Self {
        Self {
            elements: (0..100).map(|i| format!("Placeholder-{i}")).collect(),
            current_offset: scrollable::RelativeOffset::START,
            width: 0,
            height: 0,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Scrolled(offset) => {
                self.current_offset = offset; Command::none()
            },
            Message::Size(width, height) => { self.width = width; self.height = height; Command::none() },
        }
    }
}

impl Tab for LazyScroll {
    type Message = Message;

    fn title(&self) -> String {
        "Lazy Scroll".into()
    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        iced_aw::TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content = scrollable(
                column(
                    self.elements.iter().map(|s| text(s).height(Length::Units(100)).into()).collect()
                )
                .width(Length::Fill) //.height(Length::Fill)
            )
            .vertical_scroll(scrollable::Properties::new())
            .on_scroll(Message::Scrolled);
    
        container(content)
            .width(Length::Fill).height(Length::Fill)
            .padding(40)
            .center_x()
            .center_y()
            .into()
    }
}