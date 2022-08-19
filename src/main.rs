use ferris_says::say;
use std::io::{stdout, BufWriter};

pub mod guess_game;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello, world Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    guess_game::guess();
}
