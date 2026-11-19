use iced::{
    Element, Theme,
    widget::{button, column, text},
};

fn main() -> iced::Result {
    iced::application(u64::default, update, view)
        .theme(Theme::Dark)
        .centered()
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
}

fn update(value: &mut u64, message: Message) {
    match message {
        Message::Increment => *value += 1,
    }
}

fn view(value: &u64) -> Element<'_, Message> {
    column![
        text(format!("counter: {value}")).size(32),
        button("+").on_press(Message::Increment),
    ]
    .padding(20)
    .spacing(10)
    .into()
}
