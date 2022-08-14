use crate::game_trait::{DrawGame, GameLogic, InitGame, Raylib};
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

const PLAYER_CIRCLE_START_COLOR: Color = Color {
    r: 0xAC,
    g: 0xE6,
    b: 0xFE,
    a: 0xFF,
};
const PLAYER_CIRCLE_END_COLOR: Color = Color {
    r: 0xAC,
    g: 0xE6,
    b: 0xFE,
    a: 165,
};
const PLAYER_CIRCLE_RADIUS: f32 = 10.0;

const PLAYER_VISION_CIRCLE_RADIUS: f32 = 40.0;
const PLAYER_VISION_CIRCLE_COLOR: Color = Color {
    r: 0xFF,
    g: 0x9F,
    b: 0x1C,
    a: 128,
};

const PLAYER_MOVING_VELOCITY: f32 = 5.0;

///
#[derive(PartialEq)]
enum PlayerVisionDirection {
    UNKNOWN = 0,
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

//
// Player
//
struct Player {
    x: f32,
    y: f32,
    vision_direction: PlayerVisionDirection,
}

///
pub struct TronGame {
    // Raylib handle
    rl_handle: RaylibHandle,
    // Raylib thread
    rl_thread: RaylibThread,
    background: Option<Texture2D>,
    player: Player,
}


///
impl InitGame for TronGame {
    ///
    ///
    ///
    fn init_game() -> Self {
        // Player center in window
        let player = Player {
            x: (SCREEN_WIDTH / 2) as f32,
            y: (SCREEN_HEIGHT / 2) as f32,
            vision_direction: PlayerVisionDirection::UNKNOWN,
        };

        // Init window
        let (rl_handle, rl_thread) = raylib::init()
            // .fullscreen()
            // .resizable()
            // .undecorated()
            // .transparent()
            // .vsync()
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Raylib Tron Game")
            .build();

        // Set tracing log level
        set_trace_log(TraceLogLevel::LOG_DEBUG);

        let mut game = Self {
            rl_handle,
            rl_thread,
            background: None,
            player,
        };

        // Set render FPS: run at 60 frames-per-second
        game.rl_handle.set_target_fps(60);

        // Load background
        match game
            .rl_handle
            .load_texture(&game.rl_thread, "src/tron_game/res/bg.png")
        {
            Ok(texture) => game.background = Some(texture),
            Err(error) => trace_log(
                TraceLogLevel::LOG_ERROR,
                &format!("Error when loading background: {error}"),
            ),
        };

        game
    }
}

///
impl Raylib for TronGame {
    // Raylib handle
    fn get_handle(&self) -> &RaylibHandle {
        &self.rl_handle
    }
    // Raylib thread
    fn get_thread(&self) -> &RaylibThread {
        &self.rl_thread
    }
}

///
impl GameLogic for TronGame {
    ///
    /// Game logic:
    ///
    fn update_tick(&mut self) {
        //
        // Moving by vim keybingind
        //
        if self.rl_handle.is_key_down(KeyboardKey::KEY_H) {
            trace_log(TraceLogLevel::LOG_DEBUG, "Move left");

            self.player.x -= PLAYER_MOVING_VELOCITY;
            self.player.vision_direction = PlayerVisionDirection::WEST;

            if self.player.x < PLAYER_CIRCLE_RADIUS {
                self.player.x = PLAYER_CIRCLE_RADIUS;
            }
        } else if self.rl_handle.is_key_down(KeyboardKey::KEY_L) {
            trace_log(TraceLogLevel::LOG_DEBUG, "Move right");
            self.player.x += PLAYER_MOVING_VELOCITY;
            self.player.vision_direction = PlayerVisionDirection::EAST;

            if self.player.x + PLAYER_CIRCLE_RADIUS > SCREEN_WIDTH as f32 {
                self.player.x = SCREEN_WIDTH as f32 - PLAYER_CIRCLE_RADIUS;
            }
        } else if self.rl_handle.is_key_down(KeyboardKey::KEY_K) {
            trace_log(TraceLogLevel::LOG_DEBUG, "Move up");
            self.player.y -= PLAYER_MOVING_VELOCITY;
            self.player.vision_direction = PlayerVisionDirection::NORTH;

            if self.player.y < PLAYER_CIRCLE_RADIUS {
                self.player.y = PLAYER_CIRCLE_RADIUS;
            }
        } else if self.rl_handle.is_key_down(KeyboardKey::KEY_J) {
            trace_log(TraceLogLevel::LOG_DEBUG, "Move down");
            self.player.y += PLAYER_MOVING_VELOCITY;
            self.player.vision_direction = PlayerVisionDirection::SOUTH;

            if self.player.y + PLAYER_CIRCLE_RADIUS > SCREEN_HEIGHT as f32 {
                self.player.y = SCREEN_HEIGHT as f32 - PLAYER_CIRCLE_RADIUS;
            }
        }
    }
}

impl DrawGame for TronGame {
    //
    // Draw game UI
    //
    fn draw_game(&mut self) {
        //
        // No need to call `EndDrawing` manually, it will be called when
        // `RaylibDrawHandler` out of the current scope.
        //
        let mut dl = self.rl_handle.begin_drawing(&self.rl_thread);

        dl.clear_background(Color::RAYWHITE);

        //
        // Draw background on left-top with particular scale
        //
        dl.draw_texture_ex(
            self.background.as_ref().unwrap(),
            Vector2::new(0.0, 0.0),
            0.0,
            0.5,
            Color::RAYWHITE,
        );

        //
        // Draw player vision
        //
        let vision_center = Vector2::new(self.player.x, self.player.y);

        if self.player.vision_direction != PlayerVisionDirection::UNKNOWN {
            // Default east direction settings
            let mut vision_start_angle = 120.0;
            let mut vision_end_angle = 60.0;

            if self.player.vision_direction == PlayerVisionDirection::EAST {
            } else if self.player.vision_direction == PlayerVisionDirection::SOUTH {
                vision_start_angle = 30.0;
                vision_end_angle = -30.0;
            } else if self.player.vision_direction == PlayerVisionDirection::WEST {
                vision_start_angle = -120.0;
                vision_end_angle = -60.0;
            } else if self.player.vision_direction == PlayerVisionDirection::NORTH {
                vision_start_angle = 150.0;
                vision_end_angle = 210.0;
            }

            dl.draw_circle_sector(
                vision_center,
                PLAYER_VISION_CIRCLE_RADIUS,
                vision_start_angle,
                vision_end_angle,
                5,
                PLAYER_VISION_CIRCLE_COLOR,
            );
        }

        //
        // Draw player
        //
        dl.draw_circle_gradient(
            self.player.x as i32,
            self.player.y as i32,
            PLAYER_CIRCLE_RADIUS,
            PLAYER_CIRCLE_START_COLOR,
            PLAYER_CIRCLE_END_COLOR,
        );
    }
}
