use device_query::keymap::Keycode;
use device_query::{DeviceQuery, DeviceState};
use mouse_rs::types::Point;
use mouse_rs::Mouse;
use std::{thread, time::Duration, time::Instant};

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

fn looping() {
    'outerL: loop {
        let mut jiggle_counter = 1;
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();
        let sleep_duration = 2;

        if keys.contains(&Keycode::Numpad4) {
            println!("\nLooping, timer started");
            let now = Instant::now();
            loop {
                thread::sleep(Duration::new(sleep_duration, 0));
                jiggle_mouse();
                println!("jiggled {} times ", jiggle_counter);
                jiggle_counter += 1;

                let exit_key: Vec<Keycode> = device_state.get_keys();
                if exit_key.contains(&Keycode::Numpad5) {
                    let elapsed_time = now.elapsed();
                    println!(
                        "\nStopped jiggling\njiggled for {} seconds",
                        elapsed_time.as_secs()
                    );
                    continue 'outerL;
                }
            }
        }
        if keys.contains(&Keycode::Numpad6) {
            println!("\nExiting main loop");
            break;
        }
    }
}

fn jiggle_mouse() {
    let sleep_duration = 1;
    let mouse = Mouse::new();
    let first_position = mouse.get_position().unwrap();
    let second_position = Point {
        x: first_position.x + 5,
        y: first_position.y + 5,
    };
    mouse
        .move_to(second_position.x, second_position.y)
        .expect("mouse ooops");
    thread::sleep(Duration::new(sleep_duration, 0));
    mouse
        .move_to(first_position.x, first_position.y)
        .expect("another mouse ooops");
}
//TODO
// structure dir, Structs for DRY?
