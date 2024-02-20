//* To show descriptive comment in `Rust Documentation`, you must use `///` triple slashes
/// Grocery Struct
pub struct Grocery {
  name: String,
  quantity: u8,
}

/// Grocery Implementations
impl Grocery {
  /// Grocery Object Contructor
  pub fn new(name: String, quantity: u8) -> Self {
    Self { name, quantity }
  }

  pub fn check_quantity(items: Vec<Self>, name: &str) -> Option<u8> {
    //* Option is useful when a datum can be null or has a value
    for item in items {
      if item.name == name {
        return Some(item.quantity);
      }
    }

    return None;
  }
}
