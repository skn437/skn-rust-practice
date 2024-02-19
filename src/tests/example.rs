pub fn sum(num1: u8, num2: u8) -> u8 {
  num1 + num2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(sum(4, 3), 7);
  }
}
