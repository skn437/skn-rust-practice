//* The test files should contain `#[cfg(test)]` attribute
#[cfg(test)]
use super::example::sum;

#[test]
fn test() {
  assert_eq!(sum(4, 3), 7);
}
