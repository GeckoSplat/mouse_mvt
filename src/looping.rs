use crate::jiggle_mouse::jiggle_mouse;

use device_query::keymap::Keycode;
use device_query::{DeviceQuery, DeviceState};
use std::time::Instant;

pub fn looping() {
    'outerL: loop {
        let mut jiggle_counter = 1;
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::Numpad4) {
            println!("\nJiggler running, timer started :");
            let now = Instant::now();
            loop {
                jiggle_mouse();
                println!("jiggled {} times.", jiggle_counter);
                jiggle_counter += 1;

                let exit_key: Vec<Keycode> = device_state.get_keys();
                if exit_key.contains(&Keycode::Numpad5) {
                    let elapsed_time = now.elapsed();
                    println!(
                        "\nI jiggled for {} seconds !\nPaused jiggling.\n\nRunning in background....",
                        elapsed_time.as_secs()
                    );
                    continue 'outerL;
                }
            }
        }
        if keys.contains(&Keycode::Numpad6) {
            println!("\nExiting Program. Goodbye, my Leige.");
            break;
        }
    }
}
