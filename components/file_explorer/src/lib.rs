use iced::{button, scrollable, Button, Column, Container, Element, Length, Sandbox, Scrollable, Settings, Text};
use walkdir::WalkDir;

pub fn initialize_file_explorer() {
    // Run the file explorer application
    FileExplorer::run(Settings::default());
}

#[derive(Default)]
struct FileExplorer {
    scroll: scrollable::State,
    refresh_button: button::State,
    files: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    Refresh,
}

impl Sandbox for FileExplorer {
    type Message = Message;

    fn new() -> Self {
        let mut explorer = Self::default();
        explorer.load_files();
        explorer
    }

    fn title(&self) -> String {
        String::from("File Explorer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Refresh => {
                self.load_files();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let refresh_button = Button::new(&mut self.refresh_button, Text::new("Refresh"))
            .on_press(Message::Refresh);

        let file_list = self.files.iter().fold(Column::new().spacing(10), |column, file| {
            column.push(Text::new(file))
        });

        let content = Column::new()
            .padding(20)
            .spacing(20)
            .push(refresh_button)
            .push(Scrollable::new(&mut self.scroll).push(file_list));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl FileExplorer {
    fn load_files(&mut self) {
        self.files.clear();
        for entry in WalkDir::new(".").min_depth(1).max_depth(1) {
            let entry = entry.unwrap();
            self.files.push(entry.path().display().to_string());
        }
    }
}