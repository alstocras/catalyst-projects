use macroquad::prelude::*;

#[macroquad::main("empire research")]
async fn main() {
    loop {
        draw_hexagon(
            screen_width() / 2.0,
            screen_height() / 2.0,
            40.0,
            5.0,
            true,
            RED,
            BLACK,
        );

        next_frame().await;
    }
}
