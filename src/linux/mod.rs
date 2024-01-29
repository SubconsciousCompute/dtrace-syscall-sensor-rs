mod sensor {
  include!(concat!(env!("OUT_DIR"), "/sensor.skel.rs"));
}
use sensor::*;

use plain::Plain;
unsafe impl Plain for sensor_bss_types::exec_event {}

use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;
use libbpf_rs::skel::OpenSkel;
use libbpf_rs::skel::Skel;
use libbpf_rs::skel::SkelBuilder;
use libbpf_rs::RingBuffer;
use libbpf_rs::RingBufferBuilder;

use crate::utils::Process;
use crate::{utils::Record, Sensor};

pub struct Handle<'a> {
  skel: SensorSkel<'a>,
  rb: Option<RingBuffer<'a>>,
}

impl<'a> Handle<'a> {
  pub fn init() -> Result<Self, ()> {
    let skel_builder = SensorSkelBuilder::default();
    let open_skel = skel_builder.open().unwrap();

    let mut skel = open_skel.load().unwrap();
    skel.attach().unwrap();

    Ok (Handle {
      skel: skel,
      rb: None,
    })
  }
}

impl<'a> Sensor for Handle<'a> {
  fn get_process_tracker(&mut self) -> Receiver<Record> {
    let (tx, rx): (Sender<Record>, Receiver<Record>) = mpsc::channel();
    let tx = Box::leak(Box::new(tx));
    let skel = &mut self.skel;
    let mut rbb = RingBufferBuilder::new();
    let mut maps = skel.maps_mut();
    rbb.add(maps.exec_rb(), |raw| {
      let mut event = sensor_bss_types::exec_event::default();
      plain::copy_from_bytes(&mut event, raw)
        .expect("Failed to copy from bytes");
      let filename = event.filename.as_ptr();
      let filename = unsafe {std::ffi::CStr::from_ptr(filename).to_str().unwrap()};

      let process = Process::new(event.pid, filename.to_string(), event.ppid);
      let record = Record::new(process, event.timestamp as usize);
      tx.send(record).unwrap();
      0
    }).unwrap();

    self.rb = Some(rbb.build().unwrap());
    rx
  }

  fn start(&self) {
    let rb = self.rb.as_ref().unwrap();
    loop {
      rb.poll(Duration::from_millis(10)).unwrap();
    }
  }

  fn stop(&self) {
    todo!()
  }
}
