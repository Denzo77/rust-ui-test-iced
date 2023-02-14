use iced::{Command, Element, Length, widget::{container, text, column, row, Space, button}};
use iced_aw::TabLabel;
use iced_native::widget::Row;
use crate::{Icon, Tab};

const INDENT_SIZE: u16 = 20;

pub struct NestedListTab {
    internal: NestedList,
}

impl NestedListTab {
    pub fn new() -> Self {
        let internal = NestedList::with_children(vec![
            Entry::new("entry 1"),
            Entry::with_children("entry 2", &vec![
                Entry::new("2.1"),
                Entry::with_children("2.2", &vec![Entry::new("2.2.1")])
            ]),
            Entry::new("entry 3")
        ]);

        Self { internal }
    }

    pub fn update(&mut self, message: Message) -> iced::Command<Message> {
        println!("{message:?}");

        match message {
            Message::Press { id } => todo!(),
        }

        Command::none()
    }
}

impl Tab for NestedListTab {
    type Message = Message;

    fn title(&self) -> String {
        "Nested List".into()
    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {
        let entries: Element<_> = if self.internal.is_empty() {
            let content = text("No Entries").width(Length::Fill);
            container(content).into()
        } else {
            let flat_entry = |(id, entry): (usize, FlatEntry)| {
                if !entry.has_children {
                    row!(
                        Space::with_width(Length::Units(INDENT_SIZE * entry.depth)),
                        text(entry.description.clone()),
                    ).into()
                } else {
                    row!(
                        Space::with_width(Length::Units(INDENT_SIZE * entry.depth)),
                        button(text("")).on_press(Self::Message::Press { id }), // TODO: Should this just be a checkbox?
                        text(entry.description.clone()),
                    ).into()
                }
            };

            let flat_view = self.internal
                .to_vec()
                .into_iter() // Can avoid this by converting directly, or just returning iter?
                .enumerate()
                .filter(|(_, entry)| entry.visible)
                .map(flat_entry)
                .collect();
            
            column(flat_view)
                .into()
        };

        entries
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Press{ id: usize },
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowChildren {
    #[default]
    Show,
    Hide,
}


#[derive(Debug, Default, Clone)]
struct Entry {
    text: String,
    state: ShowChildren,
    children: Vec<Entry>
}

impl Entry {
    fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    fn collapse(mut self) -> Self {
        self.state = ShowChildren::Hide;

        self
    }

    fn with_children(text: &str, children: &[Entry]) -> Self {
        Self {
            text: text.into(),
            children: children.into(),
            ..Default::default()
        }
    }

    fn to_flat_view(&self, visible: bool, depth: u16) -> Vec<FlatEntry> {
        let has_children = !self.children.is_empty();
        let children_visible = visible && (self.state != ShowChildren::Hide);

        let this = FlatEntry::new(depth, visible, has_children, &self.text);

        // TODO: is there a way of doing this lazily?
        self.children.iter().fold(vec![this], |mut acc, entry| {
                acc.extend(entry.to_flat_view(children_visible, depth + 1));

                acc
            })
    }
}

#[derive(Debug, Default, Clone)]
struct NestedList {
    children: Vec<Entry>,
}

impl NestedList {
    fn with_children(children: Vec<Entry>) -> Self {
        Self { children }
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    fn to_vec(&self) -> Vec<FlatEntry> {
        self.children.iter()
            .flat_map(|entry| { entry.to_flat_view(true, 0) })
            .collect()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct FlatEntry {
    depth: u16,
    visible: bool,
    has_children: bool,
    description: String,
}

impl FlatEntry {
    fn new(depth: u16, visible: bool, has_children: bool, description: &str) -> Self {
        Self { depth, visible, has_children, description: description.into() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_entry_to_flat_view() {
        let entry = Entry::with_children("1", &vec![
            Entry::with_children("1.1", &vec![
                Entry::new("1.1.1"),
            ]),
            Entry::new("2")
        ]);

        let expected = vec![
            FlatEntry::new(0, true, true, "1"),
            FlatEntry::new(1, true, true, "1.1"),
            FlatEntry::new(2, true, false, "1.1.1"),
            FlatEntry::new(1, true, false, "2"),
        ];
        
        let flattened = entry.to_flat_view(true, 0);

        assert_eq!(flattened, expected);
    }

    #[test]
    fn flatten_nested_list_to_flat_view() {
        let nested_list = NestedList::with_children(vec![
            Entry::new("1"),
            Entry::with_children("2", &vec![
                Entry::new("2.1"),
                Entry::with_children("2.2", &vec![Entry::new("2.2.1")])
            ]),
            Entry::new("3")
        ]);

        let expected = vec![
            FlatEntry::new(0, true, false, "1"),
            FlatEntry::new(0, true, true, "2"),
            FlatEntry::new(1, true, false, "2.1"),
            FlatEntry::new(1, true, true, "2.2"),
            FlatEntry::new(2, true, false, "2.2.1"),
            FlatEntry::new(0, true, false, "3"),
        ];
        
        let flattened = nested_list.to_vec();

        assert_eq!(flattened, expected);
    }

    #[test]
    fn flatten_nested_list_to_flat_view_with_collapsed_entry() {
        let nested_list = NestedList::with_children(vec![
            Entry::new("1"),
            Entry::with_children("2", &vec![
                Entry::new("2.1"),
                Entry::with_children("2.2", &vec![Entry::new("2.2.1")])
            ]).collapse(),
            Entry::new("3")
        ]);

        let expected = vec![
            FlatEntry::new(0, true, false, "1"),
            FlatEntry::new(0, true, true, "2"),
            FlatEntry::new(1, false, false, "2.1"),
            FlatEntry::new(1, false, true, "2.2"),
            FlatEntry::new(2, false, false, "2.2.1"),
            FlatEntry::new(0, true, false, "3"),
        ];
        
        let flattened = nested_list.to_vec();

        assert_eq!(flattened, expected);
    }
}
