#[cfg(target_family = "unix")]
mod linux;
#[cfg(target_family = "windows")]
mod windows;

use std::{
    sync::mpsc::Receiver,
    thread,
};

pub struct Record {
    timestamp: usize,
    pid: i32,
}

pub trait Sensor {
    fn init() -> Self;
    fn get_file_tracker(&self) -> Receiver<Record>;
    fn get_process_tracker(&self) -> Receiver<Record>;
    fn start(&self);
    fn stop(&self);
}

fn main() {
    #[cfg(target_family = "windows")]
    let mut sensor = libdtrace_rs::wrapper::dtrace_hdl::init();

    let process_tracker = sensor.get_process_tracker();

    thread::spawn(move || {
        sensor.start();
        sensor.stop();
    });

    loop {
        match process_tracker.try_recv() {
            Ok(record) => {
                println!(
                    "pid={} executed a process at timestamp={} ",
                    record.timestamp, record.pid
                );
            }
            Err(_) => break,
        }
    }
}
