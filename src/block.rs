use piston_window::*;

pub struct Block {
    pub x: f64,
    pub y: f64,
    size: f64,
    color: types::Color,
}

impl Block {
    pub fn new(x: f64, y: f64, size: f64, color: types::Color) -> Block {
        Block {
            x: x,
            y: y,
            size: size,
            color: color
        }
    }

    pub fn show(&self, context: Context, graphics: &mut G2d) {
        rectangle(self.color, // green
                  [self.x, self.y, self.size, self.size],
                  context.transform,
                  graphics);
    }
}