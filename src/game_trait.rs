use raylib::core::{RaylibHandle, RaylibThread};

///
/// Make sure the concrete game has been initialized the `raylib`
///
pub trait Raylib {
    // Raylib handle
    fn get_handle(&self) -> &RaylibHandle;
    // Raylib thread
    fn get_thread(&self) -> &RaylibThread;
}

///
/// Create and init the game
///
pub trait InitGame {
    fn init_game() -> Self;
}

///
/// Run the game loop
///
pub trait RunGame {
    fn run(&mut self) -> &mut Self;
}

///
/// Exit game and release extra resources
///
pub trait ExitGame {
    fn exit(&mut self) -> &mut Self;
}

///
/// Update game logic on every tick
///
pub trait GameLogic {
    fn update_tick(&mut self);
}

///
/// Draw the entire game
///
pub trait DrawGame {
    fn draw_game(&mut self);
}

///
/// Default `RunGame` implementations for any concrete type
///
impl<T: Raylib + GameLogic + DrawGame> RunGame for T {
    //
    // Main game loop, exit when pressing `ESC` button or close window
    //
    fn run(&mut self) -> &mut Self {
        while !self.get_handle().window_should_close() {
            self.update_tick();
            self.draw_game();
        }

        self
    }
}

///
/// Default `ExitGame` implementations for any concrete type
///
impl<T: Raylib> ExitGame for T {
    //
    // No need to call `CloseWindow` manually, it will be called when
    // `RaylibHandler`(self.rl) out of self instance scope.
    //
    fn exit(&mut self) -> &mut Self {
        self
    }
}
