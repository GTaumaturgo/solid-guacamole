pub mod chess;
pub mod evaluation;
pub mod move_gen;
pub mod search;
pub mod server;
// Strum contains all the trait definitions
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/public/chess.html")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,create])
        .mount("/public", FileServer::from("public"))
}


// Receive requests built from JS
#[post("/")]
fn create() -> () {
    
}

