mod game_trait;
mod tron_game;

use game_trait::Game;
use tron_game::game::TronGame;

///
///
///
fn main() {
    let mut game = TronGame::init_game();

    game.run().exit();
    // let (mut rl, thread) = raylib::init()
    //     .size(640, 480)
    //     .title("Hello, World")
    //     .build();

    // while !rl.window_should_close() {
    //     let mut d = rl.begin_drawing(&thread);

    //     d.clear_background(Color::WHITE);
    //     d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    // }
}
