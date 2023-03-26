use crate::jiggle_mouse::jiggle_mouse;


use device_query::keymap::Keycode;
use device_query::{DeviceQuery, DeviceState};
use tokio::sync::oneshot::Sender;
use std::time::{Instant, Duration};

use std::sync::mpsc::{channel,Receiver};
use std::thread;



pub async fn looping() {
    

    let (_, receiver): (_, Receiver<bool>) = channel();
   

    'outerL: loop {
        let mut jiggle_counter = 1;
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();
        
        if keys.contains(&Keycode::Numpad4) {
            println!("\nJiggler running, timer started :");
            let now = Instant::now();
           'inner: loop {
                jiggle_mouse();
                println!("jiggled {} times.", jiggle_counter);
                jiggle_counter += 1;
                
                assert!(receiver.try_recv().is_err());
                println!("{:?}",(receiver.try_recv().unwrap()));
                
                
                let mut new_val : bool = false; 
                if new_val == true{
                    continue 'outerL;
                }
                else{
                    continue 'inner; 
                }
                    
                }
            }
        
        if keys.contains(&Keycode::Numpad6) {
            println!("\nExiting Program. Goodbye, my Leige.");
            break;
        }
    }}

    pub async fn key5(){
        println!("started key 5");
        'keyloop: loop{
        thread::sleep(Duration::from_secs(2));
        let now = Instant::now();
        let device_state = DeviceState::new();
        let exit_key: Vec<Keycode> = device_state.get_keys();
        let (sender, _) = channel();
        if exit_key.contains(&Keycode::Numpad5) {
        let elapsed_time = now.elapsed();
        println!(
            "\nI jiggled for {} seconds !\nPaused jiggling.\n\nRunning in background....",
            elapsed_time.as_secs()
        );
        
        thread::spawn(move|| {
            sender.send(true)//.expect("oh no true");
        
        });
        
        println!(" break at Key5",);
        break;
    }
        
        else {
            thread::spawn(move|| {
                sender.send(false)//.expect("oh no false");
            
        });
        println!("finished Key5")
    }
    }}

 // TODO - unwrap receiver.try_recv().unwrap() line 34and use its bool.