mod obstacle;
mod player;
mod state;
use bracket_lib::prelude::*;
use state::State;
const SCREEN_WIDTH: i32 = 100;
const SCREEN_HEIGHT: i32 = 100;
const FRAME_DURATION: f32 = 25.0;
fn main() -> BError {
    let context: BTerm = BTermBuilder::simple(100,100).unwrap()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
