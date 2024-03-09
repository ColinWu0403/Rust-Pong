mod lib;

use macroquad::prelude::*;
use lib::MainState;

#[macroquad::main("Pong")]
async fn main() {
    let mut main_state = MainState::new();

    loop {
        clear_background(BLACK);

        // Update game logic every frame
        for _ in 0..5 {
            main_state.update();
        }

        // Draw game objects
        main_state.draw();

        next_frame().await;
    }
}
