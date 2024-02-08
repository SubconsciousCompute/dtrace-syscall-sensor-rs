pub mod callbacks;
use libdtrace_rs::types::dtrace_handler::Buffered;
use libdtrace_rs::wrapper::dtrace_hdl;
use std::sync::mpsc::{self, Receiver, Sender};
use core::ffi::c_int;
use libdtrace_rs::DTRACE_VERSION;

use crate::utils::Record;
use crate::Sensor;

static PROGRAM: &str = r#"
    syscall:::entry
    /pid != $pid /
    {
        printf("%llu %s %d %s %d", timestamp, probefunc, pid, execname, ppid);
    }
"#;

pub struct Handle {
    handle: dtrace_hdl,
}

impl Handle {
    pub fn init() -> Self {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as c_int, 0).unwrap();

        handle.dtrace_setopt("bufsize", "4m").unwrap();
        handle.dtrace_setopt("aggsize", "4m").unwrap();
        handle.dtrace_setopt("aggrate", "1s").unwrap();
        handle.dtrace_setopt("switchrate", "1s").unwrap();
        handle.dtrace_setopt("statusrate", "1s").unwrap();

        Self {
            handle
        }
    }
}

impl Sensor for Handle {
    fn get_process_tracker(&mut self) -> Receiver<Record> {
        let (tx, rx): (Sender<Record>, Receiver<Record>) = mpsc::channel();
        let tx = Box::leak(Box::new(tx));
        let handle = &mut self.handle;
        
        handle.dtrace_register_handler(
            Buffered(Some(callbacks::buffered)),
            Some(&tx as *const _ as *mut _),
        ).unwrap();
        
        
        let prog = handle.dtrace_program_strcompile(
            PROGRAM,
            libdtrace_rs::dtrace_probespec::DTRACE_PROBESPEC_NAME,
            libdtrace_rs::DTRACE_C_ZDEFS,
            None,
        ).unwrap();
    
        handle.dtrace_program_exec(prog, None).unwrap();

        rx
    }

    fn start(&self) {
        let handle = &self.handle;
        handle.dtrace_go().unwrap();

        loop {
            handle.dtrace_sleep();
            match handle.dtrace_work(
                None,
                Some(libdtrace_rs::callbacks::chew),
                Some(libdtrace_rs::callbacks::chew_rec),
                None,
            ) {
                Ok(libdtrace_rs::dtrace_workstatus_t::DTRACE_WORKSTATUS_DONE) => {
                    break;
                }
                Ok(_) | Err(_) => (),
            }
        }
    }

    fn stop(&self) {
        let handle = &self.handle;
        handle.dtrace_stop().unwrap();
    }
}
