extern crate game_of_life;

use bbggez::ggez::conf::{FullscreenType, WindowMode};
use bbggez::ggez::event;
use bbggez::ggez::ContextBuilder;
use game_of_life::GameOfLife;

fn main() {
    let window_mode = WindowMode::default().dimensions(2000.0, 2000.0);
    // Make a Context and an EventLoop.
    let (mut context, mut event_loop) = ContextBuilder::new("Conway's Game of Life", "Brookzerker")
        .window_mode(window_mode)
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut game_of_life = GameOfLife::new(&mut context);

    // Run!
    match event::run(&mut context, &mut event_loop, &mut game_of_life) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
