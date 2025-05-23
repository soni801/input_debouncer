use lazy_static::lazy_static;
use rdev::{listen, simulate, Button, Event, EventType, Key};
use std::{sync::Mutex, thread, time::{Duration, Instant}};

const DEBOUNCE_TIME: u64 = 500;

lazy_static! {
    static ref LAST_EXECUTION: Mutex<Instant> = Mutex::new(Instant::now());
}

fn main() {
    println!("Listening for events...");

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn should_execute() -> bool {
    // Safely get the last execution time
    let mut last_execution = match LAST_EXECUTION.lock() {
        Ok(result) => result,
        Err(_) => {
            return false;
        }
    };

    // Calculate the elapsed time since the last execution
    let now = Instant::now();
    let elapsed = now.duration_since(*last_execution);

    if elapsed >= Duration::from_millis(DEBOUNCE_TIME) {
        *last_execution = now;
        true
    } else {
        println!("Stopped event execution (time since last execution: {}ms)", elapsed.as_millis());
        false
    }
}

fn send(event_type: &EventType) {
    let delay = Duration::from_millis(20);
    match simulate(event_type) {
        Ok(_) => {
            println!("Simulated {:?}.", event_type);
        },
        Err(error) => {
            println!("Could not simulate {:?}: {}", event_type, error);
        }
    }
    // Let the OS catch up (at least macOS)
    thread::sleep(delay);
}

fn callback(event: Event) {
    match event.event_type {
        EventType::ButtonPress(button) => {
            if button == Button::Unknown(1) && should_execute() {
                send(&EventType::KeyPress(Key::F12));
            }
        }
        _ => {}
    }
}
