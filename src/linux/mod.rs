mod sensor {
  include!(concat!(env!("OUT_DIR"), "/sensor.skel.rs"));
}
use sensor::*;
