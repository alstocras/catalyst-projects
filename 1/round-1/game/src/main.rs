use macroquad::prelude::*;

fn draw_hex_grid(
    hex_size: f32,
    hex_border_width: f32,
    hex_border_colour: Color,
    hex_fill_colour: Color,
) {
    let current_hex_coords: Vec2 = vec2(0.0, 0.0);
    let grid_end_coords: Vec2 = vec2(screen_width(), screen_height());

    while grid
}

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
