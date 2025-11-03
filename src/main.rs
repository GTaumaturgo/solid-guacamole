extern crate once_cell;
pub mod chess;
pub mod evaluation;
pub mod move_gen;
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

use chess::position_cache::CacheEntry;
use dashmap::DashMap;
use rocket::{
    fs::FileServer,
    get, post,
    response::{self, Redirect, Responder},
    routes,
    serde::json::Json,
    Build, Rocket, State,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

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

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/public/chess.html")
}
lazy_static! {
    pub static ref CACHE: DashMap<u64, CacheEntry> = DashMap::default();
}

#[post("/", format = "json", data = "<wrapped_uci_req>")]
async fn engine(wrapped_uci_req: Json<UciRequest>) -> Json<UciResponse> {
    let uci_req = wrapped_uci_req.into_inner();
    let req_type = uci_req.req_type.clone();
    let resp: UciResponse = if req_type == "possible_moves" {
        server::possible_moves::handle_possible_moves_request(&uci_req).await
    } else if req_type == "pos_eval" {
        server::position_eval::handle_position_eval_request(&uci_req).await
    } else {
        todo!()
    };

    Json(resp)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, engine])
        .mount("/public", FileServer::from("public"))
}
