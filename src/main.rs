use iced::{Element, Length::{self, Fill, Fixed}, Theme, alignment::{ Horizontal, Vertical}, widget::{ Text, button, checkbox, column, container, row, scrollable, text_input}};


struct Model {
    task: Task,
    task_list: Vec<Task>,
    theme: Theme
}

#[derive(Debug, Clone)]
enum Message {
    Input(String),
    CreateTask,
    DeleteTask(usize),
    SetComplete(bool, usize),
    ChangeTheme
}

#[derive(Default, Debug, Clone)]
struct Task {
    name: String,
    is_complete: bool,
}

//MAIN
pub fn main() -> iced::Result {
    //iced::run(Model::update, Model::view)
    iced::application(Model::new, Model::update, Model::view)
    .theme(|model: &Model| model.theme.clone())
    .window_size(iced::Size::new(410.0, 550.0))
    .run()
}


impl Model {
    fn new() -> Self {
        Self {
            task: Task { name: String::new(), is_complete: false },
            task_list: vec![],
            theme: Theme::Dark
        }
    }

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
            Message::SetComplete(is_complete, index ) => {
                if let Some(task) = self.task_list.get_mut(index) {
                    task.is_complete = is_complete;
                }
            }
            Message::ChangeTheme => {
                self.theme = match self.theme {
                    Theme::Dark => Theme::Light,
                    _ => Theme::Dark
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let tasks_ui = self.task_list.iter().enumerate().map(|(index, task)| {
            container(
                row![
                    Text::new(&task.name)
                        .width(Fill),

                    checkbox(task.is_complete)
                        .on_toggle(move |is_complete| Message::SetComplete(is_complete, index)),
                        
                    button("X")
                        .on_press(Message::DeleteTask(index))
                        .width(Fixed(30.0)),
                ]
                .align_y(Vertical::Center)
                .spacing(10)
            )
            .style(container::bordered_box)
            .width(Fill)
            .padding(10)
            .into()
        });

        column![
            text_input("Enter task name", &self.task.name)
                .on_input(Message::Input),

            button("Create task")
                .on_press(Message::CreateTask),

            container(
                scrollable(
                    column![].extend(tasks_ui).spacing(5))
                    .width(Fill)
                )
                .width(Fill)
                .height(Length::Fill),

            row![
                button("*")
                    .on_press(Message::ChangeTheme)
                    .style(button::secondary)
            ]          
        ]
        .padding(15)
        .spacing(5)
        .align_x(Horizontal::Center)
        .into()
    }
}
