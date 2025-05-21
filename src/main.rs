extern crate core;

use std::{env, thread};
use std::time::Duration;
use enigo::{Enigo, MouseControllable};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut enigo = Enigo::new();
    
    let mouse_movement_x: i32 = 50;
    let mouse_movement_y: i32 = 50;
    let sleep_time_secs: u64 = 5;
    
    let mouse_movement_x = args.get(1)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(mouse_movement_x);
    
    let mouse_movement_x = args.get(2)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(mouse_movement_x);
    
    
    let sleep_time_secs = args.get(3)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(sleep_time_secs);
    
    println!("Moving mouse by ({}, {}) pixels every {} seconds", mouse_movement_x, mouse_movement_y, sleep_time_secs);
    
    loop {
        enigo.mouse_move_relative(mouse_movement_x, mouse_movement_y);
        
        thread::sleep(Duration::from_secs(sleep_time_secs));
        
        enigo.mouse_move_relative(mouse_movement_x * -1, mouse_movement_y * -1);
        
        thread::sleep(Duration::from_secs(sleep_time_secs));
    }
}
