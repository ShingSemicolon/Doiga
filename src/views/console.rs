use crate::models::AnimeModel;
use super::view::View;

pub struct ConsoleView;

impl View for ConsoleView {
    fn display_message(&self, message: &str) {
        println!("{}", message);
    }

    fn display_error(&self, message: &str) {
        println!("[2;31m[1;31mError: {}[0m", message);
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

    fn display_animes(&self, animes: &Vec<AnimeModel>) {
        for (i, anime) in animes.iter().enumerate() {
            let colored_text: String;
            if i % 2 == 0 {
                colored_text = format!("[2;33m{}. {} ({})[0m", i + 1, anime.title, anime.year);
            } else {
                colored_text = format!("{}. {} ({})", i + 1, anime.title, anime.year);
            }
            println!("{}", colored_text);
        }
        println!()
    }
}