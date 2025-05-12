use crate::easy_ai::easy_ai;

use std::fs;
use std::fs::{read_to_string, File};
use std::path::Path;

const MAX_MOVES: usize = 1000;
const FILE_PATH: &str = "moves.txt";


pub fn medium_ai(data: u32) -> u32{
  let moves = logger(data);

  let (one, two, three) = count_123(&moves);

  if (one + two + three) >= 10 {
    if one >= two && one >= three {
      2
    } else if two >= three {
      3
    } else {
      1
    }
  } else {
    easy_ai()
  }

}

pub fn count_123(vec: &[u32]) -> (u32, u32, u32) {
  vec.iter().fold((0, 0, 0), |(c1,c2,c3), &v| {
    match v {
      1 => (c1+1,c2,c3),
      2 => (c1,c2+1,c3),
      3 => (c1,c2,c3+1),
      _ => (c1,c2,c3)
    }
  })
}

pub fn reader() -> Vec<u32>{
  let moves = if Path::new(FILE_PATH).exists() {
    read_to_string(FILE_PATH)
      .unwrap_or_default()
      .lines()
      .filter_map(|s| s.trim().parse::<u32>().ok())
      .collect::<Vec<_>>()
  } else {
    Vec::new()
  };
  if !Path::new(FILE_PATH).exists() {
    File::create(FILE_PATH).unwrap();
  };
  moves
}

pub fn logger(data: u32) -> Vec<u32>{
  let mut moves = reader();
  moves.push(data);

  if moves.len() > MAX_MOVES {
    moves.remove(0);
  }

  let content = moves.iter() 
    .map(|n| n.to_string())
    .collect::<Vec<_>>()
    .join("\n");

  fs::write("moves.txt", content).expect("Failed to write datas");

  moves
}