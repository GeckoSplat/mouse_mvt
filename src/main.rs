mod jiggle_mouse;
mod run;
use run::run;

fn main() {
    println!(
        "\nUp and running in background my Leige !\n\n
    Press Numpad 4 to jiggle.\n
    Press and Hold Numpad 5 to pause.\n
    Press Numpad 6 to Exit.\n"
    );

    run();
}
