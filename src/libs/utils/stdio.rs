use std::io::{stdin, Error};

use super::message::Color;

pub fn get_reader_input(prompt: &str) -> String {
  print!("{}: \n", prompt);

  let mut input: String = String::new();

  let result: Result<usize, Error> = stdin().read_line(&mut input);

  match result {
    | Ok(_) => input,
    | Err(e) => {
      print!("Error: {} \n", Color::red(e));
      get_reader_input(prompt)
    },
  }
}
