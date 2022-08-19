mod game_trait;
mod utils;
// mod tron_game;
// mod astart_grid_game;
mod find_the_shortest_path;
// mod draw_sprite_demo;

// MUST import the following traits into the current scope before the concrete
// game struct type can use the default trait implementation
use game_trait::{ExitGame, InitGame, RunGame};

// Concrete game type
// use tron_game::game::TronGame;
use find_the_shortest_path::game::FindTheShortestPathGame;


///
///
///
fn main() {
    // // let mut game = TronGame::init_game();
    let mut game = FindTheShortestPathGame::init_game();
    game.run().exit();

    // draw_sprite_demo::run();
}
