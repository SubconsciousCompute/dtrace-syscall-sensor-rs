#[cfg(target_family = "unix")]
mod linux;

#[cfg(target_family = "windows")]
mod windows;

// use std::sync::Arc;
// use std::sync::atomic::AtomicBool;
// use std::sync::mpsc::{self, Receiver, Sender};
// use std::thread;
// pub type Record = (usize, String, i32, String, i32);


fn main() -> Result<(), ()> {
    // let (tx, rx): (Sender<Record>, Receiver<Record>) = mpsc::channel();
    // let done = Arc::new(AtomicBool::new(false));
    // let (sender_done, interrupt) = (done.clone(), done.clone());
    // ctrlc::set_handler(move || {
    //     interrupt.store(true, std::sync::atomic::Ordering::SeqCst);
    // }).expect("Failed to set interrupt handler");


    Ok(())
}
