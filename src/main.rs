mod callbacks;
use libdtrace_rs::types::dtrace_handler::Buffered;
use libdtrace_rs::utils::Error;
use libdtrace_rs::wrapper::dtrace_hdl;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
pub type Record = (usize, String, i32, String, i32);

static PROGRAM: &str =
r#"
    syscall:::entry
    /pid != $pid/
    {
        printf("%llu %s %d %s %d", timestamp, probefunc, pid, execname, ppid);
    }
"#;

fn main() {
    let (tx, rx): (Sender<Record>, Receiver<Record>) = mpsc::channel();

    thread::spawn(move || -> Result<(), Error> {
        let handle =
            dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as ::core::ffi::c_int, 0)?;

        handle.dtrace_setopt("bufsize", "4m")?;
        handle.dtrace_setopt("aggsize", "4m")?;

        handle.dtrace_register_handler(
            Buffered(Some(callbacks::buffered)),
            Some(&tx as *const _ as *mut _),
        )?;

        let prog = handle.dtrace_program_strcompile(
            PROGRAM,
            libdtrace_rs::dtrace_probespec::DTRACE_PROBESPEC_NAME,
            libdtrace_rs::DTRACE_C_ZDEFS,
            None,
        )?;

        handle.dtrace_program_exec(prog, None)?;

        handle.dtrace_go()?;

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

        handle.dtrace_stop()?;

        Ok(())
    });

    loop {
        match rx.recv() {
            Ok(record) => println!(
                "Recieved: ts={}, syscall={}, pid={}, process={}, parent_pid={}",
                record.0, record.1, record.2, record.3, record.4
            ),
            Err(e) => panic!("{e}"),
        }
    }
}
