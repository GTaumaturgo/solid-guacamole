pub mod chess;
pub mod evaluation;
pub mod move_gen;
pub mod search;
pub mod ui;
// Strum contains all the trait definitions
extern crate strum;
#[macro_use]
extern crate strum_macros;


fn main() {
    // Invoke the main function of the io module
    ui::term_main();
}