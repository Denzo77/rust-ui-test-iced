use iced::{Application, Command, Element, Length, widget::{container, text, column, row, Space}};

const INDENT_SIZE: u16 = 10;

#[derive(Debug, Clone, Copy, PartialEq)]
struct TextStyle {
    size: u16,
}

struct TextStyles {
    heading: TextStyle,
    normal: TextStyle,
    // normal_empty:  TextStyle,
    // normal_icon: TextStyle,
}

const TEXT_STYLES: TextStyles = TextStyles {
    heading: TextStyle { size: 30 },
    normal: TextStyle { size: 20 },
    // normal_empty: TextStyle { size: 20, colour: [0.7; 3] },
    // normal_icon: TextStyle { size: 20, colour: [0.0; 3] },
};


pub struct NestedList {
    entries: Vec<Entry>,
}

impl Application for NestedList {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
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

        (list, Command::none())
    }

    fn title(&self) -> String {
        "Nested List".into()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
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