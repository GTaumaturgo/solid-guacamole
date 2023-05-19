use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

pub struct LichessClient {
    client: Client,
    api_key: String,
}

// #[derive(Deserialize, Serialize, Debug)]
// pub struct Game {
//     // Add fields for relevant game data, such as game ID, initial position, and moves.
// }

// impl LichessClient {
//     pub fn new(api_key: &str) -> Self {
//         let client = Client::new();
//         LichessClient {
//             client,
//             api_key: api_key.to_string(),
//         }
//     }

//     pub async fn get_game(&self, game_id: &str) -> Result<Game, Error> {
//         let url = format!("https://lichess.org/api/game/{}", game_id);
//         let response = self
//             .client
//             .get(&url)
//             .header("Authorization", format!("Bearer {}", self.api_key))
//             .send()
//             .await?;

//         let game: Game = response.json().await?;
//         Ok(game)
//     }

//     // Implement other Lichess API methods as needed.
// }
