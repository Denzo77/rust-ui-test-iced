use iced::{Command, Element, Length, widget::{container, text, column, row, Space, button}};
use iced_aw::TabLabel;
use crate::Tab;

const INDENT_SIZE: u16 = 20;

pub struct NestedListTab {
    internal: NestedList,
}

impl NestedListTab {
    pub fn new() -> Self {
        let internal = NestedList::with_children(vec![
            Entry::new("entry 1"),
            Entry::with_children("entry 2", &[
                Entry::new("2.1"),
                Entry::with_children("2.2", &[Entry::new("2.2.1")])
            ]),
            Entry::new("entry 3")
        ]);

        Self { internal }
    }

    pub fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Press { id } => self.internal.get_mut(id).map(|entry| {
                let new_state = match entry.state {
                    ShowChildren::Hide => ShowChildren::Show,
                    ShowChildren::Show => ShowChildren::Hide,
                };

                entry.state = new_state;
            }),
            Message::AddNewEntry { .. } => todo!(),
        };

        Command::none()
    }

    pub fn content(&self) -> iced::Element<'_, Message> {
        let row_height = 20;

        let entries: Element<_> = if self.internal.is_empty() {
            let content = text("No Entries").width(Length::Fill);
            container(content).into()
        } else {
            // FIXME: Only show this on mouse over
            let add_new_button = |id| button(text("+").size(10))
                .on_press(Message::AddNewEntry { id })
                .height(Length::Fill)
                .width(Length::Units(row_height));

            let flat_entry = |(id, entry): (usize, FlatEntry)| {
                if !entry.has_children {
                    row!(
                        Space::with_width(Length::Units(INDENT_SIZE * entry.depth + row_height)),
                        text(entry.description),
                        Space::with_width(Length::Fill),
                        add_new_button(id),
                    )
                } else {
                    row!(
                        Space::with_width(Length::Units(INDENT_SIZE * entry.depth)),
                        button(text(""))
                            .on_press(Message::Press { id })// TODO: Should this just be a checkbox?
                            .height(Length::Fill)
                            .width(Length::Units(row_height)),
                        text(entry.description)
                            .height(Length::Fill),
                        Space::with_width(Length::Fill),
                        add_new_button(id),
                    )
                }
            };

            let flat_view = self.internal
                .to_vec()
                .into_iter() // Can avoid this by converting directly, or just returning iter?
                .enumerate()
                .filter(|(_, entry)| entry.visible)
                .map(|entry| flat_entry(entry)
                    .width(Length::Units(200)) // TODO: make this fill
                    .height(Length::Units(row_height))
                    .into())
                .collect();
            
            column(flat_view)
                .into()
        };

        entries
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Press { id: usize },
    AddNewEntry { id: usize },
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

    fn _collapse(mut self) -> Self {
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

    fn get_mut(&mut self, id: usize) -> (usize, Option<&mut Entry>) {
        if id == 0 {
            return (0, Some(self));
        }

        let mut id = id - 1;

        for child in self.children.iter_mut() {
            let entry = child.get_mut(id);
            id = entry.0;

            if entry.1.is_some() {
                return entry
            }
        }

        (id, None)
    }

    fn _len(&self) -> usize {
        // +1 for entry in this node
        self.children.len() + 1
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

    fn get_mut(&mut self, id: usize) -> Option<&mut Entry> {
        let mut id = id;
        for child in self.children.iter_mut() {
            let entry = child.get_mut(id);
            id = entry.0;

            let entry = entry.1;
            if entry.is_some() {
                return entry
            }
        }

        None
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
            ])._collapse(),
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

    #[test]
    fn nested_list_get_correct_entry() {
        let mut nested_list = NestedList::with_children(vec![
            Entry::new("1"),
            Entry::with_children("2", &vec![
                Entry::new("2.1"),
                Entry::with_children("2.2", &vec![
                    Entry::new("2.2.1")
                ])
            ]),
            Entry::new("3")
        ]);

        let expected = vec![
            Some("1".to_string()),
            Some("2".to_string()),
            Some("2.1".to_string()),
            Some("2.2".to_string()),
            Some("2.2.1".to_string()),
            Some("3".to_string()),
            None,
        ];

        for (id, expect) in expected.iter().enumerate() {
            let entry = nested_list.get_mut(id);
            let entry = entry.map(|x| x.text.clone());

            assert_eq!(&entry, expect);
        }

    }
}
