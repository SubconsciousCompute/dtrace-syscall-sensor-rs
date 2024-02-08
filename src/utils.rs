#[derive(Debug)]
pub struct Process {
  pid: u32,
  name: String,
  parent: u32,
}

#[derive(Debug)]
pub struct Record {
  process: Process,
  timestamp: usize,
}

impl Process {
  pub fn new(pid: u32, name: String, parent: u32) -> Self {
    Process {
      pid: pid,
      name: name,
      parent: parent,
    }
  }
}

impl Record {
  pub fn new(process: Process, timestamp: usize) -> Self {
    Record {
      process: process,
      timestamp: timestamp,
    }
  }
}
