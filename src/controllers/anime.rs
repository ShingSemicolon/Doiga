use std::{error::Error, iter::Peekable};

use scraper::ElementRef;

use crate::{models::anime::AnimeModel, views::trait_view::View};

pub struct Controller<T: View> {
    view: T,
}

impl<T: View> Controller<T> {
    pub fn new(view: T) -> Self {
        Self { view }
    }

    pub fn run(&self) {
        
        let animes = self.get_animes();
        self.view.display_animes(&animes.unwrap());

    }
    fn get_html_content(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let mut input = self.view.get_user_input("Ingresa el nombre del anime");
        input = format!("https://monoschinos2.com/buscar?q={}", input.trim());
        let response = reqwest::blocking::get(&input);
        return response;
    }

    fn get_animes(&self) -> Result<Vec<AnimeModel>, Box<dyn Error>> {
        self.view.clear_screen();
        let response = self.get_html_content()?;
        let html_content = response.text()?;
        let document = scraper::Html::parse_document(&html_content);
        let animes_selector = scraper::Selector::parse("li.col.mb-3.ficha_efecto")?;
        let html_animes: Vec<ElementRef> = document.select(&animes_selector).collect();
        let animes = self.get_data(html_animes);

        return Ok(animes);
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
            animes.push(AnimeModel { title, url, year: year });
        }
        animes.sort_by_key(|anime| anime.year);
        return animes
    }


}
