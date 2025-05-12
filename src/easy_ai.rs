pub fn easy_ai() -> u32 {
  let seed = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_nanos() as u32;

  let mut rng = BASICRNG::new(seed);

  let number = rng.gen_rang(3);

  number
}

struct BASICRNG {
  state: u32,
}

impl BASICRNG {
  pub fn new(mut seed: u32) -> Self {
    if seed == 0 { seed = 0xCAFEBABE; }
    BASICRNG { state: seed }
  }

  pub fn next_u32(&mut self) -> u32 {
    let mut x = self.state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    self.state = x;
    x
  }

  pub fn gen_rang(&mut self, max: u32) -> u32 {
    assert!(max > 0);
    (self.next_u32() % max) + 1
  }
}