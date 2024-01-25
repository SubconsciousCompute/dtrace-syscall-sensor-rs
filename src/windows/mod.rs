pub mod callbacks;
use crate::{Record, Sensor};
use libdtrace_rs::dtrace_workstatus_t::DTRACE_WORKSTATUS_DONE;
use libdtrace_rs::types::dtrace_handler::Buffered;
use libdtrace_rs::wrapper::dtrace_hdl;
use std::sync::mpsc::{self, Receiver, Sender};

static PROGRAM: &str = r#"
    syscall:::entry
    /pid != $pid /
    {
        printf("%llu %s %d %s %d", timestamp, probefunc, pid, execname, ppid);
    }
"#;

impl Sensor for dtrace_hdl {
    fn init() -> dtrace_hdl {
        let handle =
            dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as ::core::ffi::c_int, 0).unwrap();
        handle.dtrace_setopt("bufsize", "4m").unwrap();
        handle.dtrace_setopt("aggsize", "4m").unwrap();
        handle.dtrace_setopt("aggrate", "1s").unwrap();
        handle.dtrace_setopt("switchrate", "1s").unwrap();
        handle.dtrace_setopt("statusrate", "1s").unwrap();

        handle
    }

    fn get_process_tracker(&self) -> Receiver<Record> {
        let (tx, rx): (Sender<Record>, Receiver<Record>) = mpsc::channel();
        self.dtrace_register_handler(
            Buffered(Some(callbacks::process_tracker_buffered)),
            Some(Box::leak(Box::new(tx)) as *const _ as *mut _),
        )
        .unwrap();
        let prog = self
            .dtrace_program_strcompile(
                PROGRAM,
                libdtrace_rs::dtrace_probespec::DTRACE_PROBESPEC_NAME,
                libdtrace_rs::DTRACE_C_ZDEFS,
                None,
            )
            .unwrap();
        self.dtrace_program_exec(prog, None).unwrap();

        rx
    }

    fn get_file_tracker(&self) -> Receiver<Record> {
        unimplemented!()
    }

    fn start(&self) {
        self.dtrace_go().unwrap();
        loop {
            self.dtrace_sleep();
            match self.dtrace_work(
                None,
                Some(libdtrace_rs::callbacks::chew),
                Some(libdtrace_rs::callbacks::chew_rec),
                None,
            ) {
                Ok(DTRACE_WORKSTATUS_DONE) => break,
                Ok(_) | Err(_) => (),
            }
        }
    }

    fn stop(&self) {
        self.dtrace_stop().unwrap();
    }
}
