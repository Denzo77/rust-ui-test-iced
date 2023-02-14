use iced::{Command, Element, Length, widget::{container, text, column, row, Space}};
use iced_aw::TabLabel;
use crate::{Icon, Tab};

const INDENT_SIZE: u16 = 10;

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

    pub fn update(&mut self, _message: Message) -> iced::Command<Message> {
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
            column(self.internal.children.iter()
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryState {
    #[default]
    Expanded,
    Collapsed,
}


#[derive(Debug, Default, Clone)]
struct Entry {
    text: String,
    state: EntryState,
    children: Vec<Entry>
}

impl Entry {
    fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    fn with_children(text: &str, children: &[Entry]) -> Self {
        Self {
            text: text.into(),
            children: children.into(),
            ..Default::default()
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

    fn to_vec(&self, depth: u16) -> Vec<FlatEntry> {
        let this = FlatEntry::new(depth, self.state, &self.text);

        // TODO: is there a way of doing this lazily?
        self.children.iter().fold(vec![this], |mut acc, entry| {
                acc.extend(entry.to_vec(depth + 1));

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
            .flat_map(|entry| { entry.to_vec(0) })
            .collect()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct FlatEntry {
    depth: u16,
    state: EntryState,
    description: String,
}

impl FlatEntry {
    fn new(depth: u16, state: EntryState, description: &str) -> Self {
        Self { depth, state, description: description.into() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_entry_to_vec() {
        let entry = Entry::with_children("1", &vec![
            Entry::with_children("1.1", &vec![
                Entry::new("1.1.1"),
            ]),
            Entry::new("2")
        ]);

        let expected = vec![
            FlatEntry::new(0, EntryState::Expanded, "1"),
            FlatEntry::new(1, EntryState::Expanded, "1.1"),
            FlatEntry::new(2, EntryState::Expanded, "1.1.1"),
            FlatEntry::new(1, EntryState::Expanded, "2"),
        ];
        
        let flattened = entry.to_vec(0);

        assert_eq!(flattened, expected);
    }

    #[test]
    fn flatten_nested_list_to_vec() {
        let nested_list = NestedList::with_children(vec![
            Entry::new("1"),
            Entry::with_children("2", &vec![
                Entry::new("2.1"),
                Entry::with_children("2.2", &vec![Entry::new("2.2.1")])
            ]),
            Entry::new("3")
        ]);

        let expected = vec![
            FlatEntry::new(0, EntryState::Expanded, "1"),
            FlatEntry::new(0, EntryState::Expanded, "2"),
            FlatEntry::new(1, EntryState::Expanded, "2.1"),
            FlatEntry::new(1, EntryState::Expanded, "2.2"),
            FlatEntry::new(2, EntryState::Expanded, "2.2.1"),
            FlatEntry::new(0, EntryState::Expanded, "3"),
        ];
        
        let flattened = nested_list.to_vec();

        assert_eq!(flattened, expected);
    }
}
