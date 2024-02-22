#[allow(dead_code)]
pub fn factorial(number: u16) -> u16 {
  if number < 1 {
    return 0;
  }

  if number == 0 || number == 1 {
    return 1;
  }

  number * factorial(number - 1)
}
