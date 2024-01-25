pub fn bump_memlock_rlimit() -> Result<(), String> {
  let rlimit = libc::rlimit {
      rlim_cur: 128 << 20,
      rlim_max: 128 << 20,
  };

  if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
      return Err("Failed to increase rlimit".to_string())
  }

  Ok(())
}
