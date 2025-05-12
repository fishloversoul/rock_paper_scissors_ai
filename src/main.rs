mod easy_ai;
mod medium_ai;
mod hard_ai;

use std::io::{self, Write};

use hard_ai::hard_ai;

//color \x1b[numberm text\x1b[0m
//green = 32, red = 31, yellow = 33
// 

fn main() {
  println!("Difficulty (easy, medium, hard, imposible) ");
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).expect("failed to read");

  let difficulty = input1.trim().to_lowercase();
  let mut ai_score = 0;
  let mut player_score= 0;
  loop {
    println!("Enter your move (rock, paper, scissors)");
    print!("player ");
    io::stdout().flush().unwrap();

    let mut input1= String::new();
    io::stdin().read_line(&mut input1).expect("failed to read");

    let input = input1.trim().to_lowercase();

    match difficulty.as_str() {
        "easy" => {
          let inpute = s_to_n(input.as_str());
          medium_ai::logger(inpute);
          let n= easy_ai::easy_ai();
          let ai = n_to_s(n);
          println!("ai {}",ai);
          let score= score(ai.to_string(), input);
          if score == 2 { player_score += 1};
          if score == 1 {ai_score += 1};
        },
        "medium" => {
          let data = s_to_n(input.as_str());
          let n = medium_ai::medium_ai(data);
          let ai = n_to_s(n);
          println!("ai {}",ai);
          let score= score(ai.to_string(), input);
          if score == 2 { player_score += 1};
          if score == 1 {ai_score += 1};
        },
        "hard" => {
          let data = s_to_n(input.as_str());
          let n = hard_ai(data);
          let ai = n_to_s(n);
          println!("ai {}",ai);
          let score= score(ai.to_string(), input);
          if score == 2 { player_score += 1};
          if score == 1 {ai_score += 1};
        }, 
        "imposible" => {
          imposible_ai(input);
          ai_score += 1;
        },
        _ => println!("pick a difficulty")
    };

    println!("ai score {}",ai_score);
    println!("player score {}", player_score);

    println!("would you like to play again (Y/N)");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("faild to read input");
    let again = input2.trim().to_lowercase();
    match again.as_str() {
      "y" => continue,
      "n" => break,
      _ => println!("put Y/N")
    }
  }
  println!("thankes for playing");
}

fn score(a:String, b:String) -> i32{
  let mut win = 0;
  match (a.as_str(), b.as_str()) {
    (a,b) if a==b => {println!("tie")},
    ("paper", "rock") | ("rock", "scissors" ) | ("scissors", "paper") => {win = 1; println!("ai wins")},
    ("rock", "paper") | ("scissors", "rock") | ("paper", "scissors") => {win = 2; println!("player wins")},
    _ => println!("aaaaaaa")
  }
  win
}

fn n_to_s(n: u32) -> &'static str {
  let s = match n {
    1 => "rock",
    2 => "paper",
    3 => "scissors",
    _ => "a"
  };
  s
}
fn s_to_n(s:&str) -> u32 {
  let n = match s {
    "rock" => 1,
    "paper" => 2,
    "scissors" => 3,
    _ => 4
  };
  n
}

fn imposible_ai(input:String){
  match input.as_str() {
    "rock" => println!("ai paper"),
    "paper" => println!("ai scissors"),
    "scissors" => println!("ai rock"),
    _ => println!("pick a move")
  }
}