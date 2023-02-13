use iced::{widget::{text_input, text, self, column, container, scrollable, button, checkbox, Text}, Application, Theme, Command, Element, Color, alignment, Length, theme, keyboard, Event, event, Font};
use iced_native::row;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};


static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);


#[derive(Debug)]
pub enum Todos {
    Loading,
    Loaded(State)
}

impl Application for Todos {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Todos::Loading,
            Command::perform(SavedState::load(), Message::Loaded)
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            Todos::Loading => false,
            Todos::Loaded(state) => state.dirty,
        };

        format!("Todos{} - Iced", if dirty { "*" } else { "" })
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            Self::Loading => {
                match message {
                    Self::Message::Loaded(Ok(state)) => {
                        *self = Self::Loaded(State {
                            input_value: state.input_value,
                            filter: state.filter,
                            tasks: state.tasks,
                            ..Default::default()
                        });
                    }
                    Self::Message::Loaded(Err(_)) => {
                        *self = Self::Loaded(State::default());
                    }
                    _ => (),
                }
                text_input::focus(INPUT_ID.clone())
            }
            Self::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    Self::Message::InputChanged(value) => {
                        state.input_value = value;
                        Command::none()
                    },
                    Self::Message::CreateTask => {
                        if !state.input_value.is_empty() {
                            state.tasks.push(Task::new(state.input_value.clone()));
                            state.input_value.clear();
                        }
                        Command::none()
                    },
                    Self::Message::FilterChanged(filter) => {
                        state.filter = filter;
                        Command::none()
                    },
                    Self::Message::TaskMessage(id, TaskMessage::Delete) => {
                        state.tasks.remove(id);
                        Command::none()
                    },
                    Self::Message::TaskMessage(id, task_message) => {
                        if let Some(task) = state.tasks.get_mut(id) {
                            let should_focus = matches!(task_message, TaskMessage::Edit);
                            task.update(task_message);
                            if should_focus {
                                let id = Task::text_input_id(id);
                                Command::batch(vec![
                                    text_input::focus(id.clone()),
                                    text_input::select_all(id),
                                ])
                            } else {
                                Command::none()
                            }
                        } else {
                            Command::none()
                        }
                    },
                    Self::Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                        Command::none()
                    }
                    Self::Message::TabPressed { shift } => {
                        if shift {
                            widget::focus_previous()
                        } else {
                            widget::focus_next()
                        }
                    },
                    _ => Command::none(),
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(SavedState {
                            input_value: state.input_value.clone(),
                            filter: state.filter,
                            tasks: state.tasks.clone(),
                        }.save(),
                        Self::Message::Saved
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self {
            Self::Loading => loading_message(),
            Self::Loaded(State { input_value, filter, tasks, .. }) => {
                let title = text("todos").width(Length::Fill).size(100).style(Color::from([0.5; 3])).horizontal_alignment(alignment::Horizontal::Center);
                let input = text_input("What needs to be done?", input_value, Message::InputChanged)
                    .id(INPUT_ID.clone())
                    .padding(15)
                    .size(30)
                    .on_submit(Self::Message::CreateTask);
                let controls = view_controls(tasks, *filter);
                let filtered_tasks = tasks.iter().filter(|task| filter.matches(task));
                let tasks: Element<_> = if filtered_tasks.count() > 0 {
                    column(tasks.iter().enumerate()
                            .filter(|(_, task)| filter.matches(task))
                            .map(|(i, task)| task.view(i).map(move |message| Self::Message::TaskMessage(i, message)))
                            .collect()
                    ).spacing(10).into()
                } else {
                    empty_message(match filter {
                        Filter::All => "You have not created a task yet...",
                        Filter::Active => "All your tasks are done.",
                        Filter::Completed => "You have not completed a task yet..."
                    })
                };

                let content = column!(title, input, controls, tasks)
                    .spacing(20)
                    .max_width(800);
                
                scrollable(container(content)
                    .width(Length::Fill)
                    .padding(40)
                    .center_x(),
                ).into()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(keyboard::Event::KeyPressed { key_code: keyboard::KeyCode::Tab, modifiers }),
                event::Status::Ignored,
            ) => Some(Message::TabPressed { shift: modifiers.shift() }),
            _ => None,
        })
    }
}

#[derive(Debug, Default)]
pub struct State {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    FilterChanged(Filter),
    TaskMessage(usize, TaskMessage),
    TabPressed { shift: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,

    #[serde(skip)]
    state: TaskState,
}

impl Task {
    fn text_input_id(id: usize) -> text_input::Id {
        text_input::Id::new(format!("task-{id}"))
    }

    fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }

    fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Completed(completed) => self.completed = completed,
            TaskMessage::Edit => self.state = TaskState::Editing,
            TaskMessage::DescriptionEdited(new) => self.description = new,
            TaskMessage::FinishedEdition => if !self.description.is_empty() { 
                self.state = TaskState::Idle;
            }
            TaskMessage::Delete => (),
        }
    }

    fn view(&self, id: usize) -> Element<TaskMessage> {
        match &self.state {
            TaskState::Idle => {
                let checkbox = checkbox(&self.description, self.completed, TaskMessage::Completed)
                    .width(Length::Fill);
                
                row!(
                    checkbox,
                    button(edit_icon())
                        .on_press(TaskMessage::Edit)
                        .padding(10)
                        .style(theme::Button::Text),
                ).spacing(20).align_items(iced::Alignment::Center).into()
            }
            TaskState::Editing => {
                let text_input = text_input("Describe your task...", &self.description, TaskMessage::DescriptionEdited)
                    .id(Self::text_input_id(id))
                    .on_submit(TaskMessage::FinishedEdition)
                    .padding(10);
                row!(
                    text_input,
                    button(row!(delete_icon(), "Delete").spacing(10))
                        .on_press(TaskMessage::Delete)
                        .padding(10)
                        .style(theme::Button::Destructive)
                ).spacing(20).align_items(iced::Alignment::Center).into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TaskState {
    Idle,
    Editing,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishedEdition,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn matches(&self, task: &Task) -> bool {
        match self {
            Filter::All => true, // returns all tasks
            Filter::Active => !task.completed, // only returns tasks that aren't completed yet
            Filter::Completed => task.completed, // only returns completed tasks
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone, Copy)]
pub enum SaveError {
    File,
    Format,
    Write,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = std::env::current_dir().unwrap_or_default();
        
        path.push("todos.json");

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();
        let mut file = async_std::fs::File::open(Self::path()).await
            .map_err(|_| LoadError::File)?;
        file.read_to_string(&mut contents).await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;
        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir).await
                .map_err(|_| SaveError::File)?;
        }

        {
            let mut file = async_std::fs::File::create(path).await
                .map_err(|_| SaveError::File)?;
            file.write_all(json.as_bytes()).await
                .map_err(|_| SaveError::Write)?;
        }

        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}


fn loading_message<'a>() -> Element<'a, Message> {
    let content = text("Loading")
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(50);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}

fn empty_message(message: &str) -> Element<'_, Message> {
    let content = text(message)
        .width(Length::Fill)
        .size(25)
        .horizontal_alignment(alignment::Horizontal::Center)
        .style(Color::from([0.7; 3]));
    container(content)
        .width(Length::Fill)
        .height(Length::Units(200))
        .center_y()
        .into()
}

fn view_controls(tasks: &[Task], current_filter: Filter) -> Element<Message> {
    let tasks_left = tasks.iter().filter(|task| !task.completed).count();

    let filter_button = |label, filter, current_filter| {
        let button_style = if filter == current_filter {
            theme::Button::Primary
        } else {
            theme::Button::Text
        };

        button(text(label)
            .size(16))
            .style(button_style)
            .on_press(Message::FilterChanged(filter))
            .padding(8)
    };

    row!(
        text(format!("{tasks_left} task{} left", if tasks_left == 1 { "" } else { "s" }))
            .width(Length::Fill)
            .size(16),
        row!(
            filter_button("All", Filter::All, current_filter),
            filter_button("Active", Filter::Active, current_filter),
            filter_button("Completed", Filter::Completed, current_filter),
        ).width(Length::Shrink).spacing(10)
    ).spacing(20).align_items(iced::Alignment::Center).into()
}

const ICONS: Font = Font::External { name: "Icons", bytes: include_bytes!("../resources/icons.ttf") };

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20)
}

fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}
