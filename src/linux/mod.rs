mod sensor {
  include!(concat!(env!("OUT_DIR"), "/sensor.skel.rs"));
}
use libbpf_rs::RingBufferBuilder;
use plain::Plain;
use sensor::*;
use libbpf_rs::skel::OpenSkel;
use libbpf_rs::skel::Skel;
use libbpf_rs::skel::SkelBuilder;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;

use crate::Record;

unsafe impl Plain for sensor_bss_types::exec_event {}

pub struct Sensor<'a> {
  skel: SensorSkel<'a>,
  ringbuffer: Option<libbpf_rs::RingBuffer<'a>>,
}

impl<'a> Sensor<'a> {
  pub fn new() -> Self {
    let skel_builder = SensorSkelBuilder::default();

    crate::utils::bump_memlock_rlimit().unwrap();
    let open_skel = skel_builder.open().unwrap();

    let mut skel = open_skel.load().unwrap();
    skel.attach().unwrap();

    Self { 
      skel,
      ringbuffer: None,
    }
  }

  pub fn get_process_tracker(&mut self) -> Receiver<Record> {
    let (tx, rx): (Sender<Record>, Receiver<Record>) = mpsc::channel();
    let tx = Box::leak(Box::new(tx));
    let mut maps = self.skel.maps_mut();
    let mut rb_builder = RingBufferBuilder::new();
    rb_builder.add(maps.exec_rb(), |raw| {
      let mut event = sensor_bss_types::exec_event::default();
      plain::copy_from_bytes(&mut event, raw).expect("Data buffer was too short");
      let filename = event.filename.as_ptr();
      let filename = unsafe {std::ffi::CStr::from_ptr(filename).to_str().unwrap()};
      let record = Record {
        pid: event.pid,
        timestamp: event.ppid as usize,
      };
      tx.send(record).unwrap();
      0
    }).unwrap();
    self.ringbuffer = Some(rb_builder.build().unwrap());

    rx
  }

  pub fn start(&mut self) {
    println!("loopin");
    while let Some(rb) = &self.ringbuffer {
      rb.poll(Duration::from_millis(100)).unwrap();
    }
  }
}


