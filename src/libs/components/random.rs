use rand::Rng;

use crate::libs::utils::{message::Color, stdio::get_reader_input};
use std::cmp::Ordering;

pub fn gen_random_number(low: i32, high: i32) -> i32 {
  //* gen_range param: (0..=10) is inclusive i.e. from 0 to 10
  //* gen_range param: (0..10) is exclusive i.e. from 0 to 9
  let random_number: i32 = rand::thread_rng().gen_range(low..=high);

  random_number
}

pub fn get_guess() {
  let random_number = gen_random_number(1, 100);

  loop {
    let output = get_reader_input("Guess a number between 1 & 100:");

    //* As .parse() returns a Result<T, E> of unknown type, any type can be inferred
    //* Then the error must be handled as usual, the compiler will auto figure out what is the error type
    //* if "integer" is inferred, then the error will be `ParseIntError`
    //* Output type is overridden here. It's called shadowing
    let output: i32 = match output.trim().parse() {
      | Ok(value) => value,
      | Err(e) => {
        print!("Error: {} \n", Color::blue(e));
        continue;
      },
    };

    //* .cmp() is used to compare values that returns an enum `Ordering` with 3 field: Equal, Less & Greater
    match output.cmp(&random_number) {
      | Ordering::Equal => {
        print!("You won! \n");
        break;
      },
      | Ordering::Greater => {
        print!("Your number is larger! \n");
        continue;
      },
      | Ordering::Less => {
        print!("You number is shorter! \n");
        continue;
      },
    }
  }
}
