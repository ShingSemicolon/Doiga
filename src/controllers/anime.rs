extern crate regex;
use std::error::Error;
use base64::{prelude::BASE64_STANDARD, Engine};
use scraper::ElementRef;
use crate::{models::{AnimeModel, PlayersModel}, views::{view::View, browser}};

pub struct Controller<T: View> {
    view: T,
}

impl<T: View> Controller<T> {
    pub fn new(view: T) -> Self {
        Self { view }
    }

    pub fn run(&self) {
            self.view.clear_screen();

            let animes = loop {
                let input = self.view.get_user_input("Ingresa el nombre del anime");
                match self.get_animes(input.as_str()) {
                    Ok(animes) if !animes.is_empty() => break animes,
                    _ =>{ 
                        self.view.display_message("Anime no encontrado.");
                        continue;
                    },
                }
            };
            
            self.view.display_animes(&animes);

            let anime = loop {
                let input = self.view.get_user_input(format!("Elige un anime  (1-{})", animes.len()).as_str());
                if let Ok(number) = input.parse::<usize>() {
                    if number > 0 && number <= animes.len() {
                        break animes.get(input.parse::<usize>().unwrap() - 1).unwrap();
                    }
                } else {
                    self.view.display_message("Ingresa un numero válido.");
                    continue;
                }
            };
            self.view.clear_screen();
            self.view.display_message(format!("Anime elegido: {}", anime.title).as_str());

            let players = loop {
                let input = self.view.get_user_input("Ingresa el episodio que quieres ver");
                match input.parse::<usize>() {
                    Ok(number) if number > 0 => break self.get_episode(anime.url.as_str(), input.as_str()),
                  
                    _ => {
                                    self.view.display_message("Ingresa un numero valido");
                                    continue;
                                }
                }
            };
            
            browser::display(anime.title.as_str(), &players);
    
       
       // TODO: Get player video and play it.
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
        let input = format!("https://monoschinos2.com/buscar?q={}", name.trim());
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