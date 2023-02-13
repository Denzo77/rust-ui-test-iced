use iced::{Command, Element, Length, widget::{container, text, column, row, Space}};
use iced_aw::TabLabel;
use crate::{Icon, Tab};

const INDENT_SIZE: u16 = 10;

pub struct NestedList {
    entries: Vec<Entry>,
}

impl NestedList {
    pub fn new() -> Self {
        let list = Self {
            entries: vec![
                Entry::new("entry 1"),
                Entry::with_children("entry 2", &vec![
                    Entry::new("2.1"),
                    Entry::with_children("2.2", &vec![Entry::new("2.2.1")])
                ]),
                Entry::new("entry 3")
            ],
        };

        list
    }

    pub fn update(&mut self, _message: Message) -> iced::Command<Message> {
        Command::none()
    }
}

impl Tab for NestedList {
    type Message = Message;

    fn title(&self) -> String {
        "Nested List".into()
    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {
        let entries: Element<_> = if self.entries.is_empty() {
            let content = text("No Entries").width(Length::Fill);
            container(content).into()
        } else {
            column(self.entries.iter()
                    .map(|entry|
                        entry.view())
                    .collect())
                .into()
        };

        entries
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Expand,
    Collapse,
}

#[derive(Debug, Default, Clone)]
struct Entry {
    text: String,
    children: Vec<Entry>
}

impl Entry {
    fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            children: Vec::new(),
        }
    }

    fn with_children(text: &str, children: &[Entry]) -> Self {
        Self {
            text: text.into(),
            children: children.into(),
        }
    }

    fn view(&self) -> Element<Message> {
        let entry = if self.children.is_empty() {
            column!(text(&self.text))
        } else {
            column!(
                text(&self.text),
                row!(
                    Space::with_width(Length::Units(INDENT_SIZE)),
                    column(self.children.iter().map(|c| c.view()).collect())
                )
            )
        };
        
        entry.into()
    }
}