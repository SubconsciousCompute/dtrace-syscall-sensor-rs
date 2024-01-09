mod callbacks;
use libdtrace_rs::wrapper::dtrace_hdl;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
pub type Record = (usize, String, i32, String);

static PROGRAM: &str = r#"
    syscall:::entry
    /pid != $pid/
    {
        printf("%llu %s %d %s", timestamp, probefunc, pid, execname);
    }
"#;

fn main() {
    let (tx, rx): (Sender<Record>, Receiver<Record>) = mpsc::channel();

    thread::spawn(move || {
        let handle =
            match dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as ::core::ffi::c_int, 0) {
                Ok(handle) => handle,
                Err(e) => panic!("{e}"),
            };

        match handle.dtrace_setopt("bufsize", "4m") {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        };

        match handle.dtrace_setopt("aggsize", "4m") {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        };

        match handle
            .dtrace_handle_buffered(Some(callbacks::buffered), Some(&tx as *const _ as *mut _))
        {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        };

        let prog = match handle.dtrace_program_strcompile(
            PROGRAM,
            libdtrace_rs::dtrace_probespec::DTRACE_PROBESPEC_NAME,
            libdtrace_rs::DTRACE_C_ZDEFS,
            None,
        ) {
            Ok(prog) => prog,
            Err(e) => panic!("{e}"),
        };

        match handle.dtrace_program_exec(prog, None) {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        };

        match handle.dtrace_go() {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        };

        loop {
            handle.dtrace_sleep();
            match handle.dtrace_work(
                None,
                Some(libdtrace_rs::callbacks::chew),
                Some(libdtrace_rs::callbacks::chew_rec),
                None,
            ) {
                Ok(libdtrace_rs::dtrace_workstatus_t::DTRACE_WORKSTATUS_DONE) => break,
                Ok(libdtrace_rs::dtrace_workstatus_t::DTRACE_WORKSTATUS_OKAY) => (),
                Ok(libdtrace_rs::dtrace_workstatus_t::DTRACE_WORKSTATUS_ERROR) => unreachable!(),
                Err(e) => (),
            }
        }

        match handle.dtrace_stop() {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        };
    });

    loop {
        match rx.recv() {
            Ok(record) => println!(
                "Recieved: ts={}, syscall={}, pid={}, process={}",
                record.0, record.1, record.2, record.3
            ),
            Err(e) => panic!("{e}"),
        }
    }
}
