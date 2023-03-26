use crate::looping::looping;
use crate::looping::key5;
pub mod jiggle_mouse;
pub mod looping;
use tokio::task;

// Runs in background once launched in terminal,
// use hotkeys to operate.


#[tokio::main]
async fn main() {
    println!(
        "\nUp and running in background my Leige !\n\n
    Press Numpad 4 to jiggle.\n
    Press and Hold Numpad 5 to pause.\n
    Press Numpad 6 to Exit.\n"
    );
    tokio::spawn(key5());
    tokio::spawn(looping());
    //looping().await;
    
}
