use std::time::Instant;

use macroquad::{prelude::*, rand, ui::root_ui};

#[macroquad::main("by-hand")]
async fn main() {
    println!("Hello, world!");

    loop {
        let start_update = Instant::now();

        // update();

        let update_elapsed = start_update.elapsed();

        // Render

        let start_render = Instant::now();

        clear_background(WHITE);

        // draw();

        let render_elapsed = start_render.elapsed();

        root_ui().label(
            None,
            format!("update: {:?}, render: {:?}", update_elapsed, render_elapsed).as_str(),
        );

        next_frame().await
    }
}
