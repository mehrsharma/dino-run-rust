use bracket_lib::prelude::*;
use dino_run::{
    dino::Dinosaur,
    cactus::Cactus,
};
use rand::Rng;

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;
const FRAME_DURATION: f32 = 75.0;

const DINO_DEAD : [u16; 1] = [ 5 ];


enum GameMode {
    Menu,
    Playing,
    End,
}


struct State {
    dino: Dinosaur,
    frame_time: f32,
    mode: GameMode,
    cacti: Vec<Cactus>,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            dino: Dinosaur::new(),
            frame_time: 0.0,
            mode: GameMode::Menu,
            cacti: Vec::new(),
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // frame logic
        ctx.cls();
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.dino.gravity_and_move()
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.dino.jump();
        }

        // player logic
        self.dino.render(ctx);
        ctx.print(0,0, "press SPACE to jump");
        ctx.print(SCREEN_WIDTH-3,0, &format!("{}", self.score));

        // obstacle logic
        let mut rng = rand::thread_rng();
        if self.cacti.is_empty() || self.dino.x + SCREEN_WIDTH - self.cacti[self.cacti.len() - 1].x > rng.gen_range(5..self.dino.x + 50){
            self.score += 1;
            self.cacti.push(Cactus::new(
                self.dino.x + SCREEN_WIDTH,
                rand::thread_rng().gen_range(0..3)
            ));
        }
        for cactus in self.cacti.iter_mut() {
            cactus.render(ctx, self.dino.x);
            if cactus.hit_obstacle(&mut self.dino) {
                self.mode = GameMode::End;
            }
        }
        for n in 0..SCREEN_WIDTH {
            ctx.set(
                n,
                SCREEN_HEIGHT - 8,
                WHITE,
                BLACK,
                to_cp437('_'),
            );
        }
        if let Some(VirtualKeyCode::Q) = ctx.key {
            self.mode = GameMode::End;
        }   
    }

    fn restart(&mut self) {
        self.dino = Dinosaur::new();
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.cacti = Vec::new();
        self.score = 0;
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
    ctx.print_centered(6, &format!("you scored {} points", state.score));
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
