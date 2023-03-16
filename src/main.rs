pub mod jiggle_mouse;
pub mod looping;
use crate::looping::looping;

// Runs in background once launched in terminal,
// use hotkeys to operate.

fn main() {
    println!(
        "\nUp and running my Leige !\n
    Press\n 
    Numpad 4 to jiggle\n
    Numpad 5 to pause\n
    Numpad 6 to Exit\n"
    );

    looping();

    println!("\nExiting Program. Goodbye, my Leige");
}
