use std::collections::BTreeSet;

use iced::{
    widget::{button, column, container, row, text, text_input, Space},
    Command, Element, Length,
};
use once_cell::sync::Lazy;

const INDENT_SIZE: u16 = 20;
static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub struct TreeViewPane {
    internal: TreeView,
}

impl TreeViewPane {
    pub fn new() -> Self {
        let internal = TreeView::with_children(vec![
            Entry::new("entry 1"),
            Entry::with_children(
                "entry 2",
                &[
                    Entry::new("2.1"),
                    Entry::with_children("2.2", &[Entry::new("2.2.1")]),
                ],
            ),
            Entry::new("entry 3"),
        ]);

        Self { internal }
    }

    pub fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::ToggleCollapse { id } => {
                if let Some(entry) = self.internal.get_mut(id) {
                    let new_state = match entry.state {
                        ShowChildren::Hide => ShowChildren::Show,
                        ShowChildren::Show => ShowChildren::Hide,
                        ShowChildren::Editing => unreachable!(),
                    };

                    entry.state = new_state;
                }
                Command::none()
            }
            Message::ToggleSelect { id } => {
                self.internal.toggle_select(id);
                println!("{:?}", self.internal.selected);
                Command::none()
            }
            Message::AddNewEntry { id } => {
                // It doesn't make sense to preserve selections when adding an entry.
                self.internal.clear_selected();
                if let Some(entry) = self.internal.get_mut(id) {
                    entry.children.push(Entry::new_empty());
                };
                // FIXME: Id's don't match so it doesn't focus properly?
                let id = FlatEntry::text_input_id(id);
                Command::batch(vec![
                    text_input::focus(id.clone()),
                    text_input::select_all(id),
                ])
            }
            Message::DescriptionEdited { id, label } => {
                if let Some(entry) = self.internal.get_mut(id) {
                    entry.text = label;
                };
                Command::none()
            }
            Message::FinishedEdit { id } => {
                if let Some(entry) = self.internal.get_mut(id) {
                    entry.state = ShowChildren::Show;
                };
                Command::none()
            }
        }
    }

    pub fn content(&self) -> iced::Element<'_, Message> {
        let row_height = 20;

        let entries: Element<_> = if self.internal.is_empty() {
            let content = text("No Entries").width(Length::Fill);
            container(content).into()
        } else {
            let flat_view = self
                .internal
                .to_vec()
                .into_iter() // Can avoid this by converting directly, or just returning iter?
                .enumerate()
                .filter(|(_, entry)| entry.visible)
                .map(|(id, entry)| entry.view(id, row_height, self.internal.selected.contains(&id)))
                .collect();

            column(flat_view)
                .width(Length::Units(200)) // TODO: make this fill
                .into()
        };

        entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    ToggleCollapse { id: usize },
    ToggleSelect { id: usize },
    AddNewEntry { id: usize },
    DescriptionEdited { id: usize, label: String },
    FinishedEdit { id: usize },
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowChildren {
    #[default]
    Show,
    Hide,
    Editing,
}

#[derive(Debug, Default, Clone)]
struct Entry {
    text: String,
    state: ShowChildren,
    children: Vec<Entry>,
}

impl Entry {
    fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    fn new_empty() -> Self {
        Self {
            text: String::new(),
            state: ShowChildren::Editing,
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

        let this = FlatEntry {
            depth,
            visible,
            has_children,
            editing: self.state == ShowChildren::Editing,
            description: &self.text,
        };

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
                return entry;
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
struct TreeView {
    children: Vec<Entry>,
    selected: BTreeSet<usize>,
}

impl TreeView {
    fn with_children(children: Vec<Entry>) -> Self {
        Self {
            children,
            selected: BTreeSet::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    fn to_vec(&self) -> Vec<FlatEntry> {
        self.children
            .iter()
            .flat_map(|entry| entry.to_flat_view(true, 0))
            .collect()
    }

    fn clear_selected(&mut self) {
        self.selected.clear()
    }

    fn toggle_select(&mut self, id: usize) {
        if self.selected.contains(&id) {
            self.selected.remove(&id);
        } else {
            self.selected.insert(id);
        }
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut Entry> {
        let mut id = id;
        for child in self.children.iter_mut() {
            let entry = child.get_mut(id);
            id = entry.0;

            let entry = entry.1;
            if entry.is_some() {
                return entry;
            }
        }

        None
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct FlatEntry<'a> {
    depth: u16,
    visible: bool,
    has_children: bool,
    editing: bool,
    description: &'a str,
}

impl<'a> FlatEntry<'a> {
    fn _new(depth: u16, visible: bool, has_children: bool, description: &'a str) -> Self {
        Self {
            depth,
            visible,
            has_children,
            editing: false,
            description,
        }
    }

    fn _new_empty(depth: u16) -> Self {
        Self {
            depth,
            visible: true,
            has_children: false,
            editing: true,
            description: "",
        }
    }

    fn text_input_id(id: usize) -> text_input::Id {
        text_input::Id::new(format!("Entry-{id}"))
    }
    // fn update(&mut self) {}

    fn view<'b>(self, id: usize, row_height: u16, selected: bool) -> Element<'b, Message> {
        let spacing = |has_button| {
            let has_button = if has_button { 0 } else { row_height };
            let width = INDENT_SIZE * self.depth + has_button;
            Space::with_width(Length::Units(width))
        };

        // FIXME: Only show this on mouse over
        let add_new_button = |id| {
            button(text("+").size(10))
                .on_press(Message::AddNewEntry { id })
                .height(Length::Fill)
                .width(Length::Units(row_height))
        };

        let element_row = |id, selected, has_children| {
            let style = if selected {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Text
            };
            row!(
                spacing(has_children),
                button(text(self.description))
                    .style(style)
                    .padding(0)
                    .on_press(Message::ToggleSelect { id })
            )
        };

        let content = if self.editing {
            let id = id;
            let text_input = text_input("Entry Name", self.description, move |label| {
                Message::DescriptionEdited { id, label }
            })
            .id(Self::text_input_id(id))
            .on_submit(Message::FinishedEdit { id });

            row!(spacing(false), text_input)
        } else if !self.has_children {
            row!(
                Space::with_width(Length::Units(row_height)),
                element_row(id, selected, true).width(Length::Fill),
                add_new_button(id),
            )
        } else {
            row!(
                button(Space::with_width(Length::Fill))
                    .on_press(Message::ToggleCollapse { id }) // TODO: Should this just be a checkbox?
                    .height(Length::Fill)
                    .width(Length::Units(row_height)),
                element_row(id, selected, false).width(Length::Fill),
                add_new_button(id),
            )
        };

        content.height(Length::Units(row_height)).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_entry_to_flat_view() {
        let entry = Entry::with_children(
            "1",
            &vec![
                Entry::with_children("1.1", &vec![Entry::new("1.1.1")]),
                Entry::new("2"),
            ],
        );

        let expected = vec![
            FlatEntry::_new(0, true, true, "1"),
            FlatEntry::_new(1, true, true, "1.1"),
            FlatEntry::_new(2, true, false, "1.1.1"),
            FlatEntry::_new(1, true, false, "2"),
        ];

        let flattened = entry.to_flat_view(true, 0);

        assert_eq!(flattened, expected);
    }

    #[test]
    fn flatten_nested_list_to_flat_view() {
        let nested_list = TreeView::with_children(vec![
            Entry::new("1"),
            Entry::with_children(
                "2",
                &vec![
                    Entry::new("2.1"),
                    Entry::with_children("2.2", &vec![Entry::new("2.2.1")]),
                ],
            ),
            Entry::new("3"),
        ]);

        let expected = vec![
            FlatEntry::_new(0, true, false, "1"),
            FlatEntry::_new(0, true, true, "2"),
            FlatEntry::_new(1, true, false, "2.1"),
            FlatEntry::_new(1, true, true, "2.2"),
            FlatEntry::_new(2, true, false, "2.2.1"),
            FlatEntry::_new(0, true, false, "3"),
        ];

        let flattened = nested_list.to_vec();

        assert_eq!(flattened, expected);
    }

    #[test]
    fn flatten_nested_list_to_flat_view_with_collapsed_entry() {
        let nested_list = TreeView::with_children(vec![
            Entry::new("1"),
            Entry::with_children(
                "2",
                &vec![
                    Entry::new("2.1"),
                    Entry::with_children("2.2", &vec![Entry::new("2.2.1")]),
                ],
            )
            ._collapse(),
            Entry::new("3"),
        ]);

        let expected = vec![
            FlatEntry::_new(0, true, false, "1"),
            FlatEntry::_new(0, true, true, "2"),
            FlatEntry::_new(1, false, false, "2.1"),
            FlatEntry::_new(1, false, true, "2.2"),
            FlatEntry::_new(2, false, false, "2.2.1"),
            FlatEntry::_new(0, true, false, "3"),
        ];

        let flattened = nested_list.to_vec();

        assert_eq!(flattened, expected);
    }

    #[test]
    fn nested_list_get_correct_entry() {
        let mut nested_list = TreeView::with_children(vec![
            Entry::new("1"),
            Entry::with_children(
                "2",
                &vec![
                    Entry::new("2.1"),
                    Entry::with_children("2.2", &vec![Entry::new("2.2.1")]),
                ],
            ),
            Entry::new("3"),
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
