extern crate piston_window;

pub mod game;
pub mod snake;
pub mod block;
pub mod food;

use game::Game;
use piston_window::*;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {

    let window: PistonWindow =
        WindowSettings::new("Hello Piston!", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new();

    game.run(window);
}
