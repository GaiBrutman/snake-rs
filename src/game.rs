use piston_window::*;
use crate::snake::Snake;

const DELAY_SECS: f64 = 0.15;

pub struct Game {
    snake: Snake,
    update_elapsed: f64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            update_elapsed: 0.0,
        }
    }

    pub fn run(&mut self, mut window: PistonWindow) {
        self.snake.set_vel(1, 0);

        while let Some(event) = window.next() {
            if let Some(_) = event.render_args() {
                window.draw_2d(&event, |context, graphics, _device| {
                    clear([0.0; 4], graphics); // Clear screen with black
                    self.snake.show(context, graphics);
                });
            }
            
            if let Some(args) = event.update_args() {
                self.update(args.dt);
            }
    
            if let Some(Button::Keyboard(key)) = event.press_args() {
                self.move_snake(key);
            }
        }
    }

    fn update(&mut self, dt: f64) {
        self.update_elapsed += dt;

        if self.update_elapsed > DELAY_SECS {
            self.snake.update();
            println!("{} seconds elapsed", self.update_elapsed);
            self.update_elapsed = 0.0;
        }
    }

    fn move_snake(&mut self, key: Key) {
        match key {
            Key::Right => {
                self.snake.set_vel(1, 0);
            },
            Key::Left => {
                self.snake.set_vel(-1, 0);
            },
            Key::Up => {
                self.snake.set_vel(0, -1);
            },
            Key::Down => {
                self.snake.set_vel(0, 1);
            },
            _ => {}
        }
    }
}