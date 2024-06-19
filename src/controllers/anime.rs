extern crate regex;
use regex::Regex;
use std::error::Error;
use base64::{prelude::BASE64_STANDARD, Engine};
use scraper::ElementRef;
use crate::{models::{AnimeModel, PlayersModel}, views::view::View};

pub struct Controller<T: View> {
    view: T,
}

impl<T: View> Controller<T> {
    pub fn new(view: T) -> Self {
        Self { view }
    }

    pub fn run(&self) {
        self.view.clear_screen();
        let input = self.view.get_user_input("Ingresa el nombre del anime");
        let Ok(animes) = self.get_animes(input.as_str()) else { return };
        self.view.display_animes(&animes);
        let input = self.view.get_user_input(format!("Elige un anime  (1-{})", animes.len()).as_str());
        let anime = animes.get(input.parse::<usize>().unwrap() - 1).unwrap();
        self.view.clear_screen();
        self.view.display_message(format!("Anime elegido: {}", anime.title).as_str());
        let input = self.view.get_user_input("Ingresa el episodio que quieres ver");
        let players = self.get_episode(anime.title.as_str(), input.as_str());
       for player in players {
           self.view.display_message(format!("{}: {}", player.name, player.data).as_str()); // Got all players and data.
       }
       // TODO: Get player video and play it. (mpv)
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
    fn get_format_url(&self, name: &str, number: &str) -> String {
        let regex = Regex::new(r"[^A-Za-z0-9 ]+").unwrap();
        let name = regex.replace_all(name, "").to_string();
        let format_text = name.split_whitespace().collect::<Vec<&str>>().join("-");
        return format!("https://monoschinos2.com/ver/{}-episodio-{}", format_text, number);
        
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
    fn get_episode(&self, name: &str, number: &str) -> Vec<PlayersModel> {
        let url = self.get_format_url(name, number);
        let response = reqwest::blocking::get(&url);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);
        let episodes_selector = scraper::Selector::parse(r#"[data-player]"#).unwrap();
        let episodes: Vec<ElementRef> = document.select(&episodes_selector).collect();
        return self.get_players(&episodes);
    }

}