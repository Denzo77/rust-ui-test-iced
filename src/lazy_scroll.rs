// use std::cell::Cell;

use iced::{widget::{scrollable, container, text, column}, Command, Length, Element, Size};
use iced_lazy::responsive;

use crate::{Tab, grid::Grid};

const DEFAULT_TILE_SIZE: u16 = 200;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Scrolled(scrollable::RelativeOffset),
}

pub struct LazyScroll {
    elements: Vec<String>,
    current_offset: scrollable::RelativeOffset,
    // size: Cell<Size>,
}

impl LazyScroll {
    pub fn new() -> Self {
        Self {
            elements: (0..100).map(|i| format!("Placeholder-{i}")).collect(),
            current_offset: scrollable::RelativeOffset::START,
            // size: Cell::new(Size::UNIT),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Scrolled(offset) => {
                self.current_offset = offset;

                // visible_elements();
                Command::none()
            },
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
        let content = |size: Size| {
            let n_columns = size.width as usize / DEFAULT_TILE_SIZE as usize;
            let visible = visible_tiles(n_columns, self.elements.len(), DEFAULT_TILE_SIZE, size, self.current_offset);

            // println!("\n update: visible: {visible:?}");

            scrollable(column!(
                    Grid::with_children(self.elements.iter().enumerate().map(|(i, s)| {
                            container(text(format!("{}: {}", s, if visible.contains(i) { "vis" } else { "hid" } ))
                                    .height(Length::Units(DEFAULT_TILE_SIZE))
                                    .width(Length::Units(DEFAULT_TILE_SIZE))
                                    .vertical_alignment(iced::alignment::Vertical::Center))
                                .style(text_style::Text)
                                .into()
                        }).collect()
                    )
                    .columns(n_columns)
                )
                .width(Length::Fill)
            )
            .vertical_scroll(scrollable::Properties::new())
            .on_scroll(Message::Scrolled)
            .into()
        };
    
        container(responsive(content))
            .width(Length::Fill).height(Length::Fill)
            .padding(40)
            .center_x()
            .center_y()
            .into()
    }
}

// inclusive bounded range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoundedRange {
    start: usize,
    end: usize,
}

impl BoundedRange {
    fn contains(&self, value: usize) -> bool {
        (self.start <= value) && (value <= self.end)
    }
}

fn widget_height_in_rows(element_height: u16, widget_size: Size) -> f32 {
    widget_size.height / element_height as f32
}

// offset is 0.0 top of row[0] and 1.0 is top of row[n-rows_in_window].
fn visible_rows(len: usize, element_height: u16, widget_size: Size, offset: scrollable::RelativeOffset) -> BoundedRange {
    let offset = offset.y;
    let widget_height = widget_height_in_rows(element_height, widget_size);
    let len = len as f32;
    let scroll_len = len - widget_height;
    let first_row = scroll_len * offset;
    // -0.5 effectively rounds down to bring the number of rows back in range.
    let last_row = (first_row + widget_height).min(len - 1.0);
    assert!(last_row < len);

    BoundedRange{ start: first_row as usize, end: last_row as usize }
}

fn visible_tiles(n_columns: usize, len: usize, element_height: u16, widget_size: Size, offset: scrollable::RelativeOffset) -> BoundedRange {
    let n_rows = len / n_columns;
    let visible = visible_rows(n_rows, element_height, widget_size, offset);

    BoundedRange { start: visible.start * n_columns, end: (visible.end + 1) * n_columns - 1 }
}

mod text_style {
    use iced::{widget::container::{StyleSheet, Appearance}, Color};

    #[derive(Debug, Clone, Copy)]
    pub struct Text;

    impl StyleSheet for Text {
        type Style = ();
        fn appearance(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                border_width: 5.0,
                border_color: Color::BLACK,
                ..Default::default()
            }
        }
    }

    impl From<Text> for iced::theme::Container {
        fn from(_style: Text) -> Self {
            iced::theme::Container::Box
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_visible_rows_in_widget() {
        let row_height = 128;
        let n_elements = 100;
        let widget_size = Size { height: 598.0, width: 500.0 };

        let offset = |y| scrollable::RelativeOffset{x: 0.0, y};

        let tests = vec![
            (offset(0.0), BoundedRange { start: 0, end: 4 }),
            (offset(0.5), BoundedRange { start: 47, end: 52 }),
            (offset(1.0), BoundedRange { start: 95, end: 99 }),
        ];

        for (offset, expected) in tests {
            let result = visible_rows(n_elements, row_height, widget_size, offset);
            assert_eq!(expected, result);
        }
    }
}
