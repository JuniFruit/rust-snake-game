use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(coord: u32) -> f64 {
    let converted = coord.try_into().unwrap_or(0.0);
    converted * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: u32, y: u32, ctx: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        ctx.transform,
        g,
    )
}

pub fn draw_rect(color: Color, x: u32, y: u32, w: u32, h: u32, ctx: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE * w as f64, BLOCK_SIZE * h as f64],
        ctx.transform,
        g,
    )
}
