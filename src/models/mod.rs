use serde::Deserialize;

pub struct AnimeModel {
    pub title: String,
    pub url: String,
    pub year: u32,
}

pub struct PlayersModel {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct ImagesUrl {
    image_url: Option<String>,
    small_image_url: Option<String>,
    large_image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Images {
    jpg: ImagesUrl,
    webp: ImagesUrl,
}

#[derive(Debug, Deserialize)]
pub struct Trailer {
    youtube_id: Option<String>,
    url: Option<String>,
    embed_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Titles {
    type_: Option<String>,
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Date {
    day: Option<u32>,
    month: Option<u32>,
    year: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Prop {
    from: Option<Date>,
    to: Option<Date>,
    string: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Aired {
    from: Option<String>,
    to: Option<String>,
    prop: Option<Prop>,
}

#[derive(Debug, Deserialize)]
pub struct Broadcast {
    day: Option<String>,
    time: Option<String>,
    timezone: Option<String>,
    string: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    mal_id: Option<u32>,
    type_: Option<String>,
    name: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnimeData {
    mal_id: Option<u32>,
    url: Option<String>,
    images: Images,
    trailer: Trailer,
    approved: Option<bool>, 
    titles: Option<Vec<Titles>>,
    pub title: String,
    title_english: Option<String>,
    title_japanese: Option<String>,
    title_synonyms: Option<Vec<String>>,
    type_: Option<String>,
    source: Option<String>,
    episodes: Option<u32>,
    status: Option<String>,
    airing: Option<bool>,
    aired: Option<Aired>,
    duration: Option<String>,
    rating: Option<String>,
    score: Option<f32>,
    scored_by: Option<u32>,
    rank: Option<u32>,
    popularity: Option<u32>,
    members: Option<u32>,
    favorites: Option<u32>,
    synopsis: Option<String>,
    background: Option<String>,
    season: Option<String>,
    year: Option<u32>,
    broadcast: Option<Broadcast>,
    producers: Option<Vec<Info>>,
    licensors: Option<Vec<Info>>,
    studios: Option<Vec<Info>>,
    genres: Option<Vec<Info>>,
    explicit_genres: Option<Vec<Info>>,
    themes: Option<Vec<Info>>,
    demographics: Option<Vec<Info>>,
}

#[derive(Debug, Deserialize)]
pub struct Items {
    count: Option<u32>,
    total: Option<u32>,
    per_page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    last_visible_page: Option<u32>,
    has_next_page: Option<bool>,
    items: Items,
}

#[derive(Debug, Deserialize)]
pub struct SeasonNow {
    pub data: Vec<AnimeData>,
    pub pagination: Pagination,
}
