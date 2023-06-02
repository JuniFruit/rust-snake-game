mod core;
mod draw;
mod game;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord;
use crate::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (50, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [to_coord(width), to_coord(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.handle_key_press(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
