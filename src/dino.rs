use bracket_lib::prelude::*;

const SCREEN_HEIGHT_FLOAT: f32 = 25.0;
const DINO_RUNNING : [u16; 2] = [ 3, 4 ];
const DINO_DUCKING : [u16; 2] = [ 6, 7 ];

pub struct Dinosaur {
    pub x: i32,
    pub y: f32,
    pub velocity: f32,
    pub frame: usize // Usize to index arrays
}

impl Dinosaur {
    pub fn new() -> Self {
        Dinosaur {
            x: 5,
            y: SCREEN_HEIGHT_FLOAT - 7.0,
            velocity: 0.0,
            frame: 0,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        if let Some(VirtualKeyCode::Down) = ctx.key {
            ctx.set_active_console(1);
            ctx.cls();
            ctx.set_fancy(
                PointF::new(5.0, self.y),
                1,
                Degrees::new(0.0),
                PointF::new(2.0, 2.0),
                WHITE,
                BLACK,
                DINO_DUCKING[self.frame]
            );
            ctx.set_active_console(0);
        } else {
            ctx.set_active_console(1);
            ctx.cls();
            ctx.set_fancy(
                PointF::new(5.0, self.y),
                1,
                Degrees::new(0.0),
                PointF::new(2.0, 2.0),
                WHITE,
                BLACK,
                DINO_RUNNING[self.frame]
            );
            ctx.set_active_console(0);
        }
    }
    
    pub fn gravity_and_move(&mut self) {
        if self.velocity < 3.0 {
            self.velocity += 0.5
        }
        self.y += self.velocity as f32;
        self.x += 1;
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.y > SCREEN_HEIGHT_FLOAT - 7.0 {
            self.y = SCREEN_HEIGHT_FLOAT - 7.0;
        }
        self.frame += 1;
        self.frame = self.frame % 2; 
    }
    pub fn jump(&mut self){
        if self.y == SCREEN_HEIGHT_FLOAT - 7.0 {
            self.velocity = -3.0;
        }
    }

    pub fn in_range(&mut self) -> bool {
        // let does_x_overlap = (obstacle.x - 1 <= self.x) && (self.x <= obstacle.x + 1);
        let hit_cactus: bool = (self.y == SCREEN_HEIGHT_FLOAT - 8.0) || (SCREEN_HEIGHT_FLOAT - 7.0 == self.y);
        hit_cactus
    }
}

