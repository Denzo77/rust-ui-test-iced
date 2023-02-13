use iced::Length;
use iced::event;
use iced::widget::Space;
use iced_native::Padding;
use iced_native::Element;
use iced_native::layout;
use iced_native::overlay;
use iced_native::widget::Tree;
use iced::widget::column;
use iced::widget::row;
use iced::widget::text;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Expand(u32),
    Collapse(u32),
    Select(u32)
}

pub struct TreeView<'a, Message, Renderer> {
    spacing: u16,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: u32,
    indentation: u16,
    children: Vec<String>,
    _phantom_message: std::marker::PhantomData<Message>,
    _phantom_renderer: std::marker::PhantomData<Renderer>,
}

impl<'a, Message, Renderer> TreeView<'a, Message, Renderer> {
    /// Creates an empty [`Column`].
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(children: Vec<Element<'a, Message, Renderer>>) -> Self {
        Self {
            spacing: 0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            indentation: 10,
            children,
            _phantom_message: std::marker::PhantomData::new(),
            _phantom_renderer: std::marker::PhantomData::new(),
        }
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the [`Padding`] of the [`Column`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Column`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Column`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Column`].
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`] .
    pub fn indentation(mut self, indentation: u16) -> Self {
        self.indentation = indentation;
        self
    }

    /// Adds an element to the [`Column`].
    pub fn push(
        mut self,
        child: impl Into<Element<'a, Message, Renderer>>,
    ) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message, Renderer> Default for TreeView<'a, Message, Renderer> {
    fn default() -> Self {
        Self::new()
    }
}


impl<'a, Message, Renderer> iced_native::Widget<Message, Renderer>
    for TreeView<'a, Message, Renderer>
where
    Renderer: iced_native::renderer::Renderer,
{
    fn children(&self) -> Vec<iced_native::widget::Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        // FIXME:
        let limits = limits
            .max_width(self.max_width)
            .width(self.width)
            .height(self.height);
        
        layout::flex::resolve(
            layout::flex::Axis::Vertical,
            renderer,
            &limits,
            self.padding,
            self.spacing as f32,
            iced_native::alignment::Alignment::Start,
            &self.children,
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: iced_native::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_native::widget::Operation<Message>,
    ) {
        // FIXME:
        operation.container(None, &mut |operation| {
            self.children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                })
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced::event::Status {
        // FIXME:
        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    shell,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced_native::mouse::Interaction {
        // FIXME:
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state,
                    layout,
                    cursor_position,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        // FIXME:
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
                state,
                renderer,
                theme,
                style,
                layout,
                cursor_position,
                viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: iced_native::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'b, Message, Renderer>> {
        overlay::from_children(&mut self.children, tree, layout, renderer)
    }
}

impl<'a, Message, Renderer> From<TreeView<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::renderer::Renderer + 'a,
{
    fn from(column: TreeView<'a, Message, Renderer>) -> Self {
        Self::new(column)
    }
}

// const INDENT_SIZE: u16 = 10;

// #[derive(Debug, Default, Clone)]
// struct Node {
//     text: String,
//     children: Vec<Node>
// }

// impl Node {
//     fn new(text: &str) -> Self {
//         Self {
//             text: text.into(),
//             children: Vec::new(),
//         }
//     }

//     fn with_children(text: &str, children: &[Node]) -> Self {
//         Self {
//             text: text.into(),
//             children: children.into(),
//         }
//     }

//     fn view(&self) -> Element<Message> {
//         let entry = if self.children.is_empty() {
//             column!(text(&self.text))
//         } else {
//             column!(
//                 text(&self.text),
//                 row!(
//                     Space::with_width(Length::Units(INDENT_SIZE)),
//                     column(self.children.iter().map(|c| c.view()).collect())
//                 )
//             )
//         };
        
//         entry.into()
//     }
// }