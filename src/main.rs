pub mod jiggle_mouse;
pub mod looping;
use crate::looping::looping;

// Runs in background once launched in terminal,
// use hotkeys to operate.

fn main() {
    println!(
        "\nUp and running in background my Leige !\n\n
    Press Numpad 4 to jiggle.\n
    Press and Hold Numpad 5 to pause.\n
    Press Numpad 6 to Exit.\n"
    );

    looping();

    println!("\nExiting Program. Goodbye, my Leige");
}
