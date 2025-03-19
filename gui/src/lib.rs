use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

pub fn initialize_gui() {

    // Run the GUI application
    CodeEditor::run(Settings::default());
}

#[derive(Default)]
struct CodeEditor {
    button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Sandbox for CodeEditor {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Rust Code Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                println!("Button was pressed!");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let button = Button::new(&mut self.button, Text::new("Press me!"))
            .on_press(Message::ButtonPressed);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(button)
            .into()
    }
}