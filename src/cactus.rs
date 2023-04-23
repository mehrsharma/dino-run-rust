use bracket_lib::prelude::*;

use crate::dino::Dinosaur;
const CACTI : [u32; 4] = [ 87, 88, 89, 90 ];
const SCREEN_HEIGHT: i32 = 25;

pub struct Cactus {
    pub x: i32,
    cactus: usize,
}
  
impl Cactus {
    pub fn new(x: i32, cactus: usize) -> Self {
        Cactus {
            x,
            cactus
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, dino_x : i32) {
        // let mut rng = rand::thread_rng();
        // let cactus: usize = rng.gen_range(0..3);
        let screen_x = self.x - dino_x + 5;

        ctx.set_active_console(1);
        ctx.set_fancy(
            PointF::new(screen_x as f32, (SCREEN_HEIGHT - 7) as f32),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            BLACK,
            CACTI[self.cactus]
        );
        ctx.set_active_console(0);
    }

    pub fn hit_obstacle(&self, dino: &mut Dinosaur) -> bool {
        let does_x_overlap = (dino.x - 1 <= self.x) && (self.x <= dino.x + 1);
        let does_y_overlap = dino.in_range();
        return does_x_overlap && does_y_overlap
    }
}