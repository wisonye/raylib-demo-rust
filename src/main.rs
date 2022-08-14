mod game_trait;
mod tron_game;

// MUST import the following traits into the current scope before the concrete
// game struct type can use the default trait implementation
use game_trait::{ExitGame, InitGame, RunGame};

// Concrete game type
use tron_game::game::TronGame;

///
///
///
fn main() {
    let mut game = TronGame::init_game();

    game.run().exit();
}
