use iced::{Application, Command, Settings, Text};
use iced_native::executor;

struct GUI;

impl Application for GUI {
    type Executor = executor::Null;

    type Message = ();

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (GUI, Command::none())
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Text::new("Hello World").into()
    }
}
fn main() {
    GUI::run(Settings::default());
}
