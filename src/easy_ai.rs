use std::time::{SystemTime, UNIX_EPOCH};

pub fn easy_ai() -> u32 {
  let seed = basic_rng(3)+1;
  seed
}

fn basic_rng(rang: u32) -> u32{
  let now = SystemTime::now();
  let time = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
  let nanos = time.as_nanos() as u64;
  let seed = (nanos % rang as u64) as u32;
  seed
}