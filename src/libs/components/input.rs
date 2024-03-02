use std::io::{stdin, Error};

use crate::libs::utils::message::Color;

#[allow(dead_code)]
pub fn get_reader_input(prompt: &str) -> String {
  print!("{} \n", prompt);

  let mut input: String = String::new();

  let result: Result<usize, Error> = stdin().read_line(&mut input);

  match result {
    | Ok(_) => input,
    | Err(e) => {
      print!("{} \n", Color::blue(e));
      get_reader_input(prompt)
    },
  }
}
