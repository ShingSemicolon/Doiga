

pub struct Anime {
    pub title: String,
    pub url: String,
    pub year: u32,
}
use std::io::{self, Write};

fn main() {
    std::process::Command::new("clear").status().unwrap();
    print!("Ingresa el nombre del anime: ");
    io::stdout().flush().unwrap(); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    let mut animes: Vec<Anime> = Vec::new();

    if html_animes.peek().is_none() {
        println!("Anime no encontrado");
        return;
    }
    for anime in html_animes {
    let title_selector = scraper::Selector::parse("h3").unwrap();
    let url_selector = scraper::Selector::parse("a").unwrap();
    let year_selector = scraper::Selector::parse("span.text-muted.fs-6").unwrap();
    let title = anime.select(&title_selector).next().unwrap().text().collect();
    let url = anime.select(&url_selector).next().unwrap().value().attr("href").unwrap().to_string();
    let year_text = anime.select(&year_selector).next().unwrap().text().collect::<String>();
    let year = year_text.parse::<u32>().unwrap();
    animes.push(Anime { title, url, year: year });
    }
  
    animes.sort_by_key(|anime| anime.year);
    for (i, anime) in animes.iter().enumerate() {
        let colored_text: String;
        if i % 2 == 0 {
            colored_text = format!("[2;33m{}. {} ({})[0m", i + 1, anime.title, anime.year);
        } else {
            colored_text = format!("{}. {} ({})", i + 1, anime.title, anime.year);
        }
        println!("{}", colored_text);
    }
    io::stdout().flush().unwrap();
    print!("Elige el anime deseado (1-{}): ", animes.len());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let index = input.trim().parse::<usize>().unwrap() - 1;
    let anime = &animes[index];
    println!("{}: {}", anime.title, anime.url);

  
}


