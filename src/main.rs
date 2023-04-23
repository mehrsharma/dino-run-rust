#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;
const FRAME_DURATION: f32 = 75.0;
const SCREEN_HEIGHT_FLOAT: f32 = 25.0;


const DINO_RUNNING : [u16; 2] = [ 3, 4 ];
const DINO_DUCKING : [u16; 2] = [ 6, 7 ];
const DINO_DEAD : [u16; 1] = [ 5 ];

enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct Player {
    x: i32,
    y: f32,
    velocity: f32,
    frame: usize // Usize to index arrays
}

impl Player {
    pub fn new(x: i32, y: f32) -> Self {
        Player {
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
}


struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5,25.0),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move()
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.jump();
        }
        self.player.render(ctx);
        ctx.print(0,0, "press SPACE to jump");

        if let Some(VirtualKeyCode::Q) = ctx.key {
            self.mode = GameMode::End;
        }   
    }

    fn restart(&mut self) {
        self.player = Player::new(5,25.0);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => main_menu(self, ctx),
            GameMode::End => dead(self, ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main_menu(state: &mut State, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "dino jump!");
    ctx.print_centered(8, "(p) play game");
    ctx.print_centered(9, "(q) quit game");
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::P => state.restart(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
        }
    }
}

fn dead(state: &mut State, ctx: &mut BTerm) {
    ctx.set_active_console(1);
    ctx.cls();
    ctx.print_centered(5, "dino run!");
    ctx.print_centered(8, "(p) play again");
    ctx.print_centered(9, "(q) quit game");
    ctx.set_fancy(
        PointF::new(5.0, 18.0),
        1,
        Degrees::new(0.0),
        PointF::new(2.0, 2.0),
        WHITE,
        BLACK,
        DINO_DEAD[0]
    );
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::P => state.restart(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
        }
    }
    ctx.set_active_console(0);
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("../resources/tileset.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/tileset.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/tileset.png")
        .with_title("dino run!")
        .with_tile_dimensions(16, 16)
        .build()?;
    main_loop(context, State::new())
}
