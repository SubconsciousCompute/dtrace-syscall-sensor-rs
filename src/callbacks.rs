pub unsafe extern "C" fn buffered(
    bufdata: *const libdtrace_rs::dtrace_bufdata_t,
    arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let tx = &mut *(arg as *mut ::std::sync::mpsc::Sender<crate::Record>);
    let msg = ::core::ffi::CStr::from_ptr((*bufdata).dtbda_buffered)
        .to_str()
        .expect("Failed to convert buffer to string");

    let record = msg.split(' ').collect::<Vec<&str>>();
    let timestamp = record[0].parse::<usize>().unwrap();
    let syscall_name = record[1].to_string();
    let pid = record[2].parse::<i32>().unwrap();
    let process_name = record[3].to_string();

    tx.send((timestamp, syscall_name, pid, process_name))
        .expect("Failed to send record");

    libdtrace_rs::DTRACE_HANDLE_OK as ::core::ffi::c_int
}
