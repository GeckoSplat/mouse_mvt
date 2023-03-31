use crate::jiggle_mouse::jiggle_mouse;
use device_query::{keymap::Keycode, DeviceQuery, DeviceState};
use std::{sync::mpsc, thread, time::Instant};

pub fn run() {
    'outer: loop {
        let (sender, receiver) = mpsc::channel::<String>();
        let mut jiggle_counter = 1;
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::Numpad6) {
            println!("\nExiting Program. Goodbye, my Leige.");
            break;
        }

        if keys.contains(&Keycode::Numpad4) {
            let now = Instant::now();
            println!("\nJiggler running, timer started :\n");

            'inner: loop {

                loop {
                    let device_state = DeviceState::new();
                    let exit_key: Vec<Keycode> = device_state.get_keys();
                    let sender2 = sender.clone();

                    thread::spawn(move || {
                        let msg = format!("you caught me !");
                        sender2.send(msg).expect("arrrgg!, sender gone wrong");
                        drop(sender2);
                    });

                    if exit_key.contains(&Keycode::Numpad5) {
                        let elapsed_time = now.elapsed();
                        println!(
                            "\nI jiggled for {} seconds !\nPaused jiggling.\n\nRunning in background....",
                            elapsed_time.as_secs()
                        );
                        continue 'outer;
                    }

                    while let Ok(_msg) = receiver.recv() {
                        //println!("thread active {msg:?}");
                        jiggle_mouse();
                        println!("\njiggled {} times..jiggler running.....\n", jiggle_counter);
                        jiggle_counter += 1;

                        continue 'inner;
                    }
                }
            }
        }
    }
}
