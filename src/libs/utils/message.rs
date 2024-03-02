use console::style;
use std::fmt::Display;

#[derive(Debug)]
pub struct Color;

impl Color {
  pub fn red<T: Display>(message: T) -> String {
    format!("{}", style(message).red())
  }

  pub fn blue<T: Display>(message: T) -> String {
    format!("{}", style(message).blue())
  }
}
