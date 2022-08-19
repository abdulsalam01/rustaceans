use ferris_says::say;
use std::io::{stdout, BufWriter};

pub mod guess_game;
pub mod array_sort;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello, world Rustaceans!");
    let width = message.chars().count();
    // mutable writer and locker
    let mut writer = BufWriter::new(stdout.lock());
    // from depedency: ferris_says::says
    say(message.as_bytes(), width, &mut writer).unwrap();
    // call public function refer to other class
    guess_game::guess();
    // add an unsorted array
    let mut arr: [i32; 4] = [4, 6, 24, 3];
    // sort and pass as reference
    array_sort::sort(&mut arr);
    // print every element using ":?" format
    println!("{:?}", arr);
}
