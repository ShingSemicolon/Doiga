
pub struct AnimeModel {
    pub title: String,
    pub url: String,
    pub year: u32,
}

impl AnimeModel {
    pub fn new(title: String, url: String, year: u32) -> Self {
        Self {title, url, year}
    }
}