//! A small task list, staged for the README hero images.
//!
//! ```sh
//! cargo run --example hero -- --headless --control /tmp/hero.sock &
//! cargo run --example hero -- ctl /tmp/hero.sock tree
//! ```

use iced::widget::{Space, button, checkbox, column, container, row, text, text_input};
use iced::{Alignment, Element, Length, Theme};

fn main() -> iced::Result {
    let app = iced::application(Tasks::new, Tasks::update, Tasks::view)
        .theme(|_: &Tasks| Theme::TokyoNight)
        .window_size((560.0, 480.0));
    iced_remote::Remote::new(app).run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filter {
    All,
    Active,
    Done,
}

#[derive(Debug, Clone)]
enum Message {
    Draft(String),
    Add,
    Toggle(usize, bool),
    Filter(Filter),
    ClearDone,
}

struct Task {
    title: String,
    done: bool,
}

struct Tasks {
    draft: String,
    tasks: Vec<Task>,
    filter: Filter,
}

impl Tasks {
    fn new() -> Self {
        let task = |title: &str, done| Task {
            title: title.to_owned(),
            done,
        };
        Self {
            draft: String::new(),
            tasks: vec![
                task("Sketch the settings screen", true),
                task("Wire up the save button", true),
                task("Check contrast in light mode", false),
                task("Record the demo video", false),
            ],
            filter: Filter::All,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Draft(draft) => self.draft = draft,
            Message::Add => {
                let title = self.draft.trim();
                if !title.is_empty() {
                    self.tasks.push(Task {
                        title: title.to_owned(),
                        done: false,
                    });
                }
                self.draft.clear();
            }
            Message::Toggle(i, done) => self.tasks[i].done = done,
            Message::Filter(filter) => self.filter = filter,
            Message::ClearDone => self.tasks.retain(|t| !t.done),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let done = self.tasks.iter().filter(|t| t.done).count();
        let header = row![
            text("Tasks").size(28),
            Space::new().width(Length::Fill),
            text(format!("{done} of {} done", self.tasks.len())).size(14),
        ]
        .align_y(Alignment::Center);

        let add = row![
            text_input("What needs doing?", &self.draft)
                .id("new")
                .on_input(Message::Draft)
                .on_submit(Message::Add)
                .padding(10),
            button(text("Add")).on_press(Message::Add).padding([10, 18]),
        ]
        .spacing(8);

        let list = column(
            self.tasks
                .iter()
                .enumerate()
                .filter(|(_, t)| match self.filter {
                    Filter::All => true,
                    Filter::Active => !t.done,
                    Filter::Done => t.done,
                })
                .map(|(i, t)| {
                    checkbox(t.done)
                        .label(&t.title)
                        .on_toggle(move |done| Message::Toggle(i, done))
                        .size(18)
                        .spacing(12)
                        .into()
                }),
        )
        .spacing(14);

        let tab = |label: &'static str, filter: Filter| {
            let style = if self.filter == filter {
                button::primary
            } else {
                button::text
            };
            button(text(label))
                .on_press(Message::Filter(filter))
                .style(style)
        };
        let footer = row![
            tab("All", Filter::All),
            tab("Active", Filter::Active),
            tab("Done", Filter::Done),
            Space::new().width(Length::Fill),
            button(text("Clear done"))
                .on_press(Message::ClearDone)
                .style(button::secondary),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        container(column![header, add, list, Space::new().height(Length::Fill), footer].spacing(24))
            .padding(32)
            .into()
    }
}
