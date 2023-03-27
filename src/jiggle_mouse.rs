use mouse_rs::types::Point;
use mouse_rs::Mouse;
use std::{thread, time::Duration};

pub fn jiggle_mouse() {
    let sleep_duration = 2;
    let sleep_duration_nanos = 250000000;
    let mouse = Mouse::new();
    // amount of movement set here in pixels
    let first_position = mouse.get_position().unwrap();
    let second_position = Point {
        x: first_position.x + 0,
        y: first_position.y + 1,
    };
    //wait between movements
    thread::sleep(Duration::new(sleep_duration, 0));
    mouse
        .move_to(second_position.x, second_position.y)
        .expect("mouse ooops");
    //wait between jiggles
    thread::sleep(Duration::new(0, sleep_duration_nanos));
    mouse
        .move_to(first_position.x, first_position.y)
        .expect("another mouse ooops");
}
