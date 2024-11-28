extern crate regex;
use std::{error::Error, io::stdout};
use base64::{prelude::BASE64_STANDARD, Engine};
use crossterm::{cursor::{self, MoveTo}, event::{self, Event, KeyCode}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{self, Clear, ClearType}};
use scraper::ElementRef;
use crate::{controllers::anime_client::AnimeClient, models::{AnimeModel, PlayersModel}, views::view::View};

pub struct Controller<T: View> {
    view: T,
}

impl<T: View> Controller<T> {
    pub fn new(view: T) -> Self {
        Self { view }
    }

    pub async fn run(&self) {
            let anime = AnimeClient::new();
            let season_now = anime.get_season_now().await.unwrap().data;
            let mut selected_index = 0;
            terminal::enable_raw_mode().unwrap();
            loop {
                // Limpia la pantalla y dibuja las opciones
                execute!(stdout(), Clear(ClearType::All)).unwrap();
                for (index, anime) in season_now.iter().enumerate() {
                    if index == selected_index {
                        // Marca la opción seleccionada
                        execute!(
                            stdout(),
                            MoveTo(0, index as u16),
                            SetForegroundColor(Color::Black),
                            SetBackgroundColor(Color::White),
                            Print(format!("> {}", anime.title)),
                            ResetColor
                        ).unwrap();
                    } else {
                        execute!(stdout(), MoveTo(0, index as u16), Print(format!("  {}\r\n", anime.title))).unwrap();
                    }
                }
        
                if let Event::Key(key_event) = event::read().unwrap() {
                    match key_event.code {
                        KeyCode::Up => {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            print!("{}", season_now.len());
                            if selected_index < season_now.len() - 1 {
                                selected_index += 1;
                            }
                        }
                        KeyCode::Enter => {
                            // Maneja la selección
                            if season_now[selected_index].title == "Salir" {
                                break;
                            } else {
                                execute!(
                                    stdout(),
                                    Clear(ClearType::All),
                                    Print(format!("Seleccionaste: {}\n", season_now[selected_index].title))
                                ).unwrap();
                                std::thread::sleep(std::time::Duration::from_secs(2));
                            }
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        
            // Restaurar el estado del terminal
            execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen).unwrap();
            terminal::disable_raw_mode().unwrap();
        }



    fn get_data(&self, document: Vec<ElementRef>) -> Vec<AnimeModel> {
        let mut animes: Vec<AnimeModel> = Vec::new();
        for anime in document {
            let title_selector = scraper::Selector::parse("h3").unwrap();
            let url_selector = scraper::Selector::parse("a").unwrap();
            let year_selector = scraper::Selector::parse("span.text-muted.fs-6").unwrap();
            let title = anime.select(&title_selector).next().unwrap().text().collect();
            let url = anime.select(&url_selector).next().unwrap().value().attr("href").unwrap().to_string();
            let year_text = anime.select(&year_selector).next().unwrap().text().collect::<String>();
            let year = year_text.parse::<u32>().unwrap();
            animes.push(AnimeModel { title, url, year });
        }
        animes.sort_by_key(|anime| anime.year);
        return animes
    }
    fn get_animes(&self, name: &str) -> Result<Vec<AnimeModel>, Box<dyn Error>> {
        let input = format!("https://monoschinos2.com/buscar?q={}", name.trim()); // https://nyaa.si/
        let response = reqwest::blocking::get(&input);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);
        let animes_selector = scraper::Selector::parse("li.col.mb-3.ficha_efecto").unwrap();
        let html_animes: Vec<ElementRef> = document.select(&animes_selector).collect();
        let animes = self.get_data(html_animes);

        return Ok(animes);
    }
    fn get_format_url(&self, url: &str, number: &str) -> String {
        let response = reqwest::blocking::get(url);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);
        let episodes_selector = scraper::Selector::parse(r#"a[href*="1"]"#).unwrap();
        let episode = document.select(&episodes_selector).next().unwrap();
        let url = episode.value().attr("href").unwrap().to_string();
        let regex = regex::Regex::new(r"\d+$").unwrap();
        let url = regex.replace(&url, number).to_string();
        return url;
    }
    fn decode_base64(episode: &ElementRef) -> String {
        let data_player = episode.value().attr("data-player").unwrap();
        let vec_data = BASE64_STANDARD.decode(data_player).unwrap();
        return String::from_utf8(vec_data).unwrap();
    }
    fn get_players(&self, episodes: &Vec<ElementRef>) -> Vec<PlayersModel> {
        let mut players: Vec<PlayersModel> = Vec::new();
        for episode in episodes {
            let text = episode.text().collect::<Vec<&str>>()[0];
            let link = Controller::<T>::decode_base64(episode);
            players.push(PlayersModel { name: text.to_string(), data: link });
        }
        return players;    
    }
    fn get_episode(&self, url: &str, number: &str) -> Vec<PlayersModel> {
        let url = self.get_format_url(url, number);
        let response = reqwest::blocking::get(&url);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);
        let episodes_selector = scraper::Selector::parse(r#"[data-player]"#).unwrap();
        let episodes: Vec<ElementRef> = document.select(&episodes_selector).collect();
        return self.get_players(&episodes);
    }

}