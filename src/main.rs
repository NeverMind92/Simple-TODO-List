use iced::{Element, Length::{Fill, Fixed}, Padding, Renderer, alignment::{self, Horizontal, Vertical}, widget::{Text, bottom, button, column, container, row, space, text_input}};

#[derive(Default)]
struct Model {
    task: Task,
    task_list: Vec<Task>,
}

#[derive(Debug, Clone)]
enum Message {
    Input(String),
    CreateTask,
    DeleteTask(usize),
}

#[derive(Default, Debug, Clone)]
struct Task {
    name: String,
    is_complete: bool,
}

impl Model {
    fn update(&mut self, message: Message) {
        match message {
            Message::Input(content) => {
                self.task.name = content;
            }
            Message::CreateTask => {
                if !self.task.name.is_empty() {
                    self.task_list.push(Task {
                        name: self.task.name.clone(),
                        is_complete: false,
                    });
                    self.task.name = String::new();
                }
            }
            Message::DeleteTask(index) => {
                self.task_list.remove(index);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let tasks_ui = self.task_list.iter().enumerate().map(|(index, task)| {
            container(
                row![
                    Text::new(&task.name)
                        .width(Fill),
                    button("X")
                        .on_press(Message::DeleteTask(index))
                        .width(Fixed(30.0))
                ]
                .align_y(Vertical::Center)
                .spacing(10)
            )
            .width(Fill)
            .padding(10)
            .into()
        });

        column![
            text_input("Enter task name", &self.task.name)
                .on_input(Message::Input),
            button("Create task")
                .on_press(Message::CreateTask),
        ]
        .padding(15)
        .spacing(5)
        .align_x(Horizontal::Center)
        .extend(tasks_ui)
        .into()
    }
}

pub fn main() -> iced::Result {
    iced::run(Model::update, Model::view)
}
