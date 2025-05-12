use crate::medium_ai::{count_123, reader, logger};

const WINDOW_SIZE: usize = 2;

pub fn hard_ai(data: u32) -> u32{
  let fmoves = pattern_find();
  let (one, two, three) = count_123(&fmoves);
  logger(data);

  println!("{:?}",fmoves);

  if one >= two && one >= three {
    2
  } else if two >= three {
    3
  } else {
    1
  }
}

fn pattern_find() -> Vec<u32>{
  let moves = reader();
  println!("{:?}",moves);
  let recent_pattern = &moves[moves.len().saturating_sub(WINDOW_SIZE)..];

  let mut following_move = Vec::new();

  for i in 0..moves.len().saturating_sub(WINDOW_SIZE) {
    if &moves[i..i + WINDOW_SIZE] == recent_pattern {
      if let Some(&next) = moves.get(i + WINDOW_SIZE) {
        following_move.push(next);
      }
    }
  };

  println!("{:?}",following_move);

  following_move
}