use crate::game_trait::{DrawGame, GameLogic, InitGame, Raylib};
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const SCREEN_RECT: Rectangle = Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

///
///
///
enum PathNodeType {
    Normal,
    StartNode,
    DestinationNode,
    PathResultNode,
}

///
///
///
struct PathNode {
    node_type: PathNodeType,
    pos: Vector2,
}

///
///
///
enum GameStatus {
    // Draw all nodes and AStar path if exists
    Normal,
    // Draw edge from the current node to mouse position, click to create new
    // node.
    CreatingNodes,
    // Highlighted the hover node, click to select and save start node.
    SelectStartNode,
    // Highlighted the hover node, click to select and save destination node.
    // Then run the `AStar` algorightm to calculate the shortest path.
    SelectDestinationNode,
}

///
pub struct FindTheShortestPathGame {
    // Raylib handle
    rl_handle: RaylibHandle,
    // Raylib thread
    rl_thread: RaylibThread,

    shortest_path: Option<Vec<PathNode>>,
    path_nodes: Option<Vec<PathNode>>,
    start_node: Option<PathNode>,
    destination_node: Option<PathNode>,
    background: Color,
}

///
impl InitGame for FindTheShortestPathGame {
    ///
    ///
    ///
    fn init_game() -> Self {
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
            shortest_path: None,
            path_nodes: None,
            start_node: None,
            destination_node: None,
            background: Color::from_hex("011627").unwrap(),
        };

        // Set render FPS: run at 60 frames-per-second
        game.rl_handle.set_target_fps(60);

        game
    }
}

///
impl Raylib for FindTheShortestPathGame {
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
impl GameLogic for FindTheShortestPathGame {
    ///
    /// Game logic:
    ///
    fn update_tick(&mut self) {}
}

impl DrawGame for FindTheShortestPathGame {
    //
    // Draw game UI
    //
    fn draw_game(&mut self) {
        //
        // No need to call `EndDrawing` manually, it will be called when
        // `RaylibDrawHandler` out of the current scope.
        //
        let mut dl = self.rl_handle.begin_drawing(&self.rl_thread);

        dl.clear_background(self.background);
    }
}
