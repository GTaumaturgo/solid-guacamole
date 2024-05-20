extern crate once_cell;
pub mod chess;
pub mod evaluation;
pub mod move_gen;
pub mod search;
pub mod server;
use move_gen::MovesMap;
use once_cell::sync::Lazy;

// static RUNTIME: Lazy<EngineRuntime> = Lazy::new(|| { runtime::EngineRuntime::new()});

// static RUNTIME: &EngineRuntime = &EngineRuntime::new();

// Strum contains all the trait definitions
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate rocket;
extern crate serde;

use rocket::{
    fs::FileServer,
    get, post,
    response::{Redirect, Responder},
    routes,
    serde::json::Json,
    Build, Rocket,
};

use serde::{Deserialize, Serialize};

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
}
// ThenChange:
// JS UciRequest.

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/public/chess.html")
}

#[post("/", format = "json", data = "<wrapped_uci_req>")]
fn engine(wrapped_uci_req: Json<UciRequest>) -> Json<UciResponse> {
    let uci_req = wrapped_uci_req.into_inner();
    let req_type = uci_req.req_type.clone();
    let resp: UciResponse = if req_type == "possible_moves" {
        server::handle_possible_moves_request(&uci_req)
    } else if req_type == "best_moves" {
        UciResponse {
            best_moves: "qqq".to_string(),
            possible_moves: "qqq".to_string(),
        }
    } else {
        UciResponse {
            best_moves: "qqq".to_string(),
            possible_moves: "qqq".to_string(),
        }
    };

    Json(resp)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, engine])
        .mount("/public", FileServer::from("public"))
}
