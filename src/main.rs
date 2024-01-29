use std::sync::mpsc::Receiver;

mod utils;
use utils::Record;
#[cfg(target_family = "unix")]
mod linux;

#[cfg(target_family = "windows")]
mod windows;

pub trait Sensor {
    fn get_process_tracker(&self) -> Receiver<Record>;
    fn start(&self);
    fn stop(&self);
}

fn main() {
    
}
