use super::trait_view::View;

pub struct ConsoleView;

impl View for ConsoleView {
    fn display_message(&self, message: &str) {
        println!("{}", message);
    }

    fn get_user_input(&self, message: &str) -> String {
        use std::io::{self, Write};

        print!("{}: ", message);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }

    fn clear_screen(&self) {
        if cfg!(windows) {
            std::process::Command::new("cls").status().unwrap();
        } else {
            std::process::Command::new("clear").status().unwrap();
        }
    }
}
