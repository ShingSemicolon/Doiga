use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use crate::models::SeasonNow;
#[derive(Debug)]
pub struct AnimeClient {
    client: Client,
    base_url: &'static str,
    version: &'static str,
}

impl AnimeClient {
    // Constructor
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.jikan.moe",
            version: "v4",
        }
    }

    // Método para construir URLs completas
    fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}/{}", self.base_url, self.version, endpoint)
    }

    // Ejemplo de método que actúa como endpoint , user_id: u32
    pub async fn get_season_now(&self) -> Result<SeasonNow, Box<dyn Error>> {
        let url = self.build_url("seasons/now");
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let season_now = response.json::<SeasonNow>().await?;
            Ok(season_now)
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }

    // Otro ejemplo de método que actúa como endpoint
    // pub async fn create_post(&self, title: &str, content: &str) -> Result<PostResponse, Box<dyn Error>> {
    //     let url = self.build_url("posts");
    //     let payload = serde_json::json!({
    //         "title": title,
    //         "content": content,
    //     });

    //     let response = self.client.post(&url).json(&payload).send().await?;

    //     if response.status().is_success() {
    //         let post = response.json::<PostResponse>().await?;
    //         Ok(post)
    //     } else {
    //         Err(format!("Request failed with status: {}", response.status()).into())
    //     }
    // }
}


// 

// #[tokio::main]
// async fn main() {
//     let api = APIHandler::new();

//     match api.get_user(1).await {
//         Ok(user) => println!("User: {:?}", user),
//         Err(e) => eprintln!("Error fetching user: {}", e),
//     }

//     match api.create_post("Hello, Rust!", "This is a test post.").await {
//         Ok(post) => println!("Post created: {:?}", post),
//         Err(e) => eprintln!("Error creating post: {}", e),
//     }
// }


