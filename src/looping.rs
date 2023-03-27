use crate::jiggle_mouse::jiggle_mouse;
use device_query::keymap::Keycode;
use device_query::{DeviceQuery, DeviceState};
use std::process::exit;
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::thread;

pub async fn looping() {
    println!("started fn looping");
    'outerL: loop {
        let (sender, receiver) = mpsc::channel::<String>();
        let mut jiggle_counter = 1;
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::Numpad6) {
            println!("\nExiting Program. Goodbye, my Leige.");
            exit(0x0100);
        }

        if keys.contains(&Keycode::Numpad4) {
            println!("\nJiggler running, timer started :");
            let now = Instant::now();
            'inner: loop {
                println!("inner loop started");
                loop {
                    thread::sleep(Duration::from_secs(2));
                    let now = Instant::now();
                    let device_state = DeviceState::new();
                    let exit_key: Vec<Keycode> = device_state.get_keys();
                    let sender2 = sender.clone();

                    thread::spawn(move || {
                        let msg = format!("hit me !");
                        
                        
                        sender2.send(msg).unwrap(); //.expect("oh no true");
                        drop(sender2);
                    });

                    if exit_key.contains(&Keycode::Numpad5) {
                        let elapsed_time = now.elapsed();
                        println!(
                            "\nI jiggled for {} seconds !\nPaused jiggling.\n\nRunning in background....",
                            elapsed_time.as_secs()
                        );
                        continue 'outerL;
                    }

                    while let Ok(msg) = receiver.recv() {
                        println!("while let active {msg:?}");

                        jiggle_mouse();
                        println!("jiggled {} times.", jiggle_counter);
                        jiggle_counter += 1;

                        continue 'inner;
                    }
                }
            }
        }
    }
}


// TODO - work out waits and if order correct . put timer in.
