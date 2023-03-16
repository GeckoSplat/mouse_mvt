use mouse_rs::types::Point;
use mouse_rs::Mouse;
use std::{thread, time::Duration};

pub fn jiggle_mouse() {
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
