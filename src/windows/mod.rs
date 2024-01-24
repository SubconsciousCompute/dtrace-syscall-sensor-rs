pub mod callbacks;
use crate::{Record, Sensor};
use std::sync::mpsc::{Sender, Receiver};
use libdtrace_rs::wrapper::dtrace_hdl;

impl Sensor for dtrace_hdl {
    fn new() -> Self {
        todo!()
    }

    fn start(&mut self, channel: Sender<Record>) {
        
    }

    fn stop(&mut self) {

    }
}


// use libdtrace_rs::types::dtrace_handler::Buffered;
// use libdtrace_rs::utils::Error;

// static PROGRAM: &str = r#"
//     syscall:::entry
//     /pid != $pid /
//     {
//         printf("%llu %s %d %s %d", timestamp, probefunc, pid, execname, ppid);
//     }
// "#;



    // let handle = dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as ::core::ffi::c_int, 0)?;

    // handle.dtrace_setopt("bufsize", "4m")?;
    // handle.dtrace_setopt("aggsize", "4m")?;
    // handle.dtrace_setopt("aggrate", "1s")?;
    // handle.dtrace_setopt("switchrate", "1s")?;
    // handle.dtrace_setopt("statusrate", "1s")?;

    // handle.dtrace_register_handler(
    //     Buffered(Some(callbacks::buffered)),
    //     Some(&tx as *const _ as *mut _),
    // )?;

    // let prog = handle.dtrace_program_strcompile(
    //     PROGRAM,
    //     libdtrace_rs::dtrace_probespec::DTRACE_PROBESPEC_NAME,
    //     libdtrace_rs::DTRACE_C_ZDEFS,
    //     None,
    // )?;

    // handle.dtrace_program_exec(prog, None)?;

    // handle.dtrace_go()?;


    // let sender = thread::spawn(move || -> Result<(), Error> {
    //     while !done.load(std::sync::atomic::Ordering::SeqCst) {
    //         handle.dtrace_sleep();
    //         match handle.dtrace_work(
    //             None,
    //             Some(libdtrace_rs::callbacks::chew),
    //             Some(libdtrace_rs::callbacks::chew_rec),
    //             None,
    //         ) {
    //             Ok(libdtrace_rs::dtrace_workstatus_t::DTRACE_WORKSTATUS_DONE) => done.store(true, std::sync::atomic::Ordering::SeqCst),
    //             Ok(_) | Err(_) => (),
    //         }
    //     }

    //     handle.dtrace_stop()?;
    //     Ok(())
    // });


    // while !sender_done.load(std::sync::atomic::Ordering::SeqCst) {
    //     match rx.recv() {
    //         Ok(record) => {
    //             println!(
    //                 "timestamp={} syscall={} pid={} process={} parent_pid={}",
    //                 record.0, record.1, record.2, record.3, record.4
    //             );
    //         }
    //         Err(_) => break,
    //     }
    // }


    // sender.join()
    //       .expect("The sender thread has panicked")?;

