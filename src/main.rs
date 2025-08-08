extern crate once_cell;
pub mod chess;
pub mod evaluation;
pub mod move_gen;
pub mod opening_book;
pub mod server;

// static RUNTIME: Lazy<EngineRuntime> = Lazy::new(|| { runtime::EngineRuntime::new()});

// static RUNTIME: &EngineRuntime = &EngineRuntime::new();

// Strum contains all the trait definitions
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate rocket;
extern crate serde;
use once_cell::sync::Lazy;

use crate::chess::position::Position; // Added for Position
use rocket::{
    fs::FileServer,
    get, post,
    response::{self, Redirect, Responder},
    routes,
    fairing::AdHoc,
    State,
    serde::json::Json,
    Build, Rocket,
};

use serde::{Deserialize, Serialize};

use crate::opening_book::{OpeningBook, load_book, get_book_move}; // Added get_book_move

static GLOBAL_OPENING_BOOK: Lazy<OpeningBook> = Lazy::new(|| {
    load_book("book.bin").unwrap_or_else(|err| {
        eprintln!("Failed to load opening book: {:?}. Using empty book.", err);
        OpeningBook::new()
    })
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
// IfChange:
pub struct UciRequest {
    pub p_to_move: String,
    pub board: String,
    pub req_type: String,
    pub timeout: u32,
}

#[derive(Responder, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[response(status = 200, content_type = "json")]

pub struct UciResponse {
    pub best_moves: String,
    #[response(ignore)]
    pub possible_moves: String,
    #[response(ignore)]
    pub pos_score: String,
}
// ThenChange:
// JS UciRequest.

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/public/chess.html")
}

#[post("/", format = "json", data = "<wrapped_uci_req>")]
fn engine(
    wrapped_uci_req: Json<UciRequest>,
    book: &State<OpeningBook> // Access the book from managed state
) -> Json<UciResponse> {
    let uci_req = wrapped_uci_req.into_inner();
    let req_type = uci_req.req_type.clone();

    let resp: UciResponse = if req_type == "possible_moves" {
        server::possible_moves::handle_possible_moves_request(&uci_req)
    } else if req_type == "pos_eval" {
        server::position_eval::handle_position_eval_request(&uci_req)
    } else {
        // Assuming this branch is for finding the best move
        let mut current_position = Position::from_uci(&uci_req);
        current_position.compute_zobrist_hash(); // Ensure hash is up-to-date
        let position_key = current_position.position_info.zobrist_hash;

        if let Some(book_move) = get_book_move(book, position_key) {
            // We found a move in the book!
            // TODO:
            // 1. Parse `book_move.move_uci` (e.g., "e2e4") into your engine's `BitboardMove` struct.
            //    This requires a function like `BitboardMove::from_uci_string(&book_move.move_uci, &current_position)`
            //    The current_position might be needed to disambiguate pawn moves or determine piece type for the move.
            // 2. Validate this `BitboardMove` against `current_position.legal_continuations()`.
            // 3. If valid, return it in `UciResponse`.

            // Placeholder response:
            println!("Book move found: {}", book_move.move_uci);
            // This is a placeholder. You need to convert `book_move.move_uci`
            // to whatever format your engine uses internally and then format it
            // for the UciResponse.
            // The `best_moves` field in `UciResponse` likely expects a UCI string.
            UciResponse {
                best_moves: book_move.move_uci, // Assuming best_moves takes a single UCI string
                possible_moves: "".to_string(), // Or fill appropriately
                pos_score: "Book".to_string(),  // Indicate it's a book move
            }
        } else {
            // No book move, proceed to search (actual search logic is not shown here)
            // This part needs to be implemented with your actual search call.
            // For now, returning a dummy response.
            println!("No book move found, proceeding to search (not implemented here).");
            UciResponse {
                best_moves: "a1a2".to_string(), // Dummy best move
                possible_moves: "".to_string(),
                pos_score: "Searched (dummy)".to_string(),
            }
            // TODO: Call your actual search function here:
            // e.g., search::find_best_move(&current_position, uci_req.timeout)
        }
    };

    Json(resp)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(
            load_book("book.bin").unwrap_or_else(|err| {
                eprintln!(
                    "Error loading opening book 'book.bin': {:?}. Using an empty book.",
                    err
                );
                OpeningBook::new()
            })
        )
        .mount("/", routes![index, engine])
        .mount("/public", FileServer::from("public"))
}
