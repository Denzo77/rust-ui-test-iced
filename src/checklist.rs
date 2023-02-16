

use iced::{Application, widget::{text_input, text, column, scrollable, container, checkbox, row, button, Text}, Command, Renderer, Color, alignment, Element, Length, theme, Font};
use once_cell::sync::Lazy;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug, Clone, Copy, PartialEq)]
struct TextStyle {
    size: u16,
    colour: [f32; 3],
}

impl TextStyle {
    fn colour(&self) -> Color {
        self.colour.into()
    }
}

struct TextStyles {
    heading: TextStyle,
    normal: TextStyle,
    normal_empty:  TextStyle,
    normal_icon: TextStyle,
}

const TEXT_STYLES: TextStyles = TextStyles {
    heading: TextStyle { size: 30, colour: [0.5; 3] },
    normal: TextStyle { size: 20, colour: [0.0; 3] },
    normal_empty: TextStyle { size: 20, colour: [0.7; 3] },
    normal_icon: TextStyle { size: 20, colour: [0.0; 3] },
};

#[derive(Debug, Default)]
pub struct Checklist {
    input_value: String,
    entries: Vec<Entry>
}

impl Application for Checklist {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Checklist".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::InputChanged(value) => {
                self.input_value = value;
                Command::none()
            },
            Self::Message::CreateEntry => {
                if !self.input_value.is_empty() {
                    self.entries.push(Entry::new(self.input_value.clone()));
                    self.input_value.clear();
                }
                Command::none()
            },
            Self::Message::Entry { id, message } => update_entry(&mut self.entries, id, message),
            // Self::Message => ,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        let title = text("entries")
            .width(iced::Length::Fill)
            .size(TEXT_STYLES.heading.size)
            .style(TEXT_STYLES.heading.colour())
            .horizontal_alignment(alignment::Horizontal::Center);
        
        let input = text_input("New Entry", &self.input_value, Self::Message::InputChanged)
            .id(INPUT_ID.clone())
            .padding(10)
            .size(TEXT_STYLES.normal.size)
            .on_submit(Self::Message::CreateEntry);
        
        let entries: Element<_> = if self.entries.is_empty() {
            empty_message("No Entries")
        } else {
            column(self.entries.iter()
                    .enumerate()
                    .map(|(id, entry)|
                        entry.view(id).map(move |message| Self::Message::Entry { id, message }))
                    .collect())
                .spacing(10)
                .into()
        };
        
        let content = column!(title, input, entries)
            .spacing(20)
            .max_width(800);
        
        scrollable(container(content)
                .width(iced::Length::Fill)
                .padding(20)
                .center_x())
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    CreateEntry,
    Entry { id: usize, message: EntryMessage }
}

#[derive(Default, Debug, Clone)]
struct Entry {
    description: String,
    checked: EntryChecked,
    state: EntryState,
}

impl Entry {
    fn new(description: String) -> Self {
        Self {
            description,
            checked: EntryChecked::default(),
            state: EntryState::default(),
        }
    }

    fn text_input_id(id: usize) -> text_input::Id {
        text_input::Id::new(format!("Entry-{id}"))
    }

    fn update(&mut self, message: EntryMessage) {
        match message {
            EntryMessage::Checked(checked) => self.checked = checked,
            EntryMessage::Edit => self.state = EntryState::Editing,
            EntryMessage::DescriptionEdited(new) => self.description = new,
            EntryMessage::FinishedEdit => if !self.description.is_empty() { self.state = EntryState::Idle }
            EntryMessage::Delete => (),
        }
    }

    fn view(&self, id: usize) -> Element<EntryMessage> {
        match &self.state {
            EntryState::Idle => {
                let checkbox = checkbox(&self.description, matches!(self.checked, EntryChecked::Checked), |b| if b { EntryMessage::Checked(EntryChecked::Checked) } else { EntryMessage::Checked(EntryChecked::Unchecked) })
                    .width(Length::Fill);
                row!(checkbox,
                        button(edit_icon())
                            .on_press(EntryMessage::Edit)
                            .padding(5)
                            .style(theme::Button::Text))
                    .spacing(10) 
                    .align_items(alignment::Alignment::Center)
                    .into()
            },
            EntryState::Editing => {
                let text_input = text_input("Entry name", &self.description, EntryMessage::DescriptionEdited)
                    .id(Self::text_input_id(id))
                    .on_submit(EntryMessage::FinishedEdit)
                    .padding(5);
                row!(
                        text_input,
                        button(row!(delete_icon(), "Delete").spacing(10))
                            .on_press(EntryMessage::Delete)
                            .padding(5)
                            .style(theme::Button::Destructive))
                    .spacing(10)
                    .align_items(alignment::Alignment::Center)
                    .into()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EntryMessage {
    Checked(EntryChecked),
    Edit,
    DescriptionEdited(String),
    FinishedEdit,
    Delete,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum EntryState {
    #[default]
    Idle,
    Editing,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryChecked {
    #[default]
    Unchecked,
    _Partial,
    Checked,
}

fn update_entry(entries: &mut Vec<Entry>, id: usize, message: EntryMessage) -> Command<Message> {
    match message {
        EntryMessage::Delete => {
            entries.remove(id);
            Command::none()
        },
        EntryMessage::Edit => {
            entries[id].update(message);
            let id = Entry::text_input_id(id);
            Command::batch(vec![
                text_input::focus(id.clone()),
                text_input::select_all(id),
            ])
        },
        _ => {
            entries[id].update(message);
            Command::none()
        },
    }
}

const ICONS: Font = Font::External { name: "Icons", bytes: include_bytes!("../resources/icons.ttf") };

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(TEXT_STYLES.normal_icon.size)
}

fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}

fn empty_message(message: &str) -> Element<'_, Message> {
    let content = text(message)
        .width(Length::Fill)
        .size(TEXT_STYLES.normal_empty.size)
        .style(TEXT_STYLES.normal_empty.colour())
        .horizontal_alignment(alignment::Horizontal::Center);
    container(content)
        .width(Length::Fill)
        .height(Length::Units(200))
        .center_y()
        .into()
}
