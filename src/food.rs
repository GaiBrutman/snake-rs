// use rand::Rng;
// use piston_window::*;
// use crate::block::Block;

// const FOOD_SIZE: f64 = 20.0;
// const RED: types::Color = [1.0, 0.0, 0.0, 1.0];

// pub struct Food(Block);

// impl Food {
//     pub fn new(map_width: u32, map_height: u32) -> Food {
//         let mut rng = rand::thread_rng();

//         Food {
//             0: Block::new(
//                 rng.gen_range(0.0, map_width as f64),
//                 rng.gen_range(0.0, map_height as f64),
//                 FOOD_SIZE,
//                 RED,
//             )
//         }
//     }

//     pub fn show(&self, context: Context, graphics: &mut G2d) {
//         self.0.show(context, graphics);
//     }
// }
