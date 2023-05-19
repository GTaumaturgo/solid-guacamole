mod bitboard;
mod zobrist;
mod transposition_table;
mod lichess_client;
mod common;

// use crate::lichess_client::play_on_lichess;
use tokio;

#[tokio::main]
async fn main() {
    // Add your Lichess API key and the game ID you want to play
    let api_key = "your_api_key";
    let game_id = "your_game_id";

    // let lichess_client = LichessClient::new(api_key);
    // if let Err(e) = lichess_client.play_on_lichess(game_id).await {
    //     eprintln!("Error playing game on Lichess: {}", e);
    // }
}