use std::{sync::mpsc::Receiver, thread};

mod utils;
use utils::Record;
#[cfg(target_family = "unix")]
mod linux;
#[cfg(target_family = "unix")]
use linux::Handle;

#[cfg(target_family = "windows")]
mod windows;
#[cfg(target_family = "windows")]
use windows::Handle;

pub trait Sensor {
    fn get_process_tracker(&mut self) -> Receiver<Record>;
    fn start(&self);
    fn stop(&self);
}

fn main() {
    let mut handle = Handle::init().unwrap();

    let tracker = handle.get_process_tracker();

    thread::spawn(move || {
        handle.start();
    });

    loop {
        match tracker.recv() {
            Ok(record) => println!("{:?}", record),
            Err(_) => break,
        }
    }
}
