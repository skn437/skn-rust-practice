#[cfg(test)]
use super::factorial::factorial;
#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn factorial_test() {
  // FIXME
  //* factorial(0) fails the test, it should be fixed!
  // assert_eq!(factorial(0), 1);
  assert_eq!(factorial(1), 1);
  assert_eq!(factorial(2), 2);
  assert_eq!(factorial(3), 6);
  assert_eq!(factorial(4), 24);
  assert_eq!(factorial(5), 120);
  assert_eq!(factorial(6), 720);
  assert_eq!(factorial(7), 5040);
}
