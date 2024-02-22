#[derive(Debug)]
pub enum AuthorizationStatus {
  Access,
  Deny,
}

#[derive(Debug)]
pub enum ProtectedLocation {
  All,
  Office,
  Warehouse,
}

impl ProtectedLocation {
  pub fn required_access_level(&self) -> u16 {
    match self {
      | Self::All => 1000,
      | Self::Office => 800,
      | Self::Warehouse => 500,
    }
  }
}

#[derive(Debug)]
pub struct Employee {
  name: String,
}

impl Employee {
  fn new(name: String) -> Self {
    Self { name }
  }
}

#[derive(Debug)]
pub struct KeyCard {
  access_level: u16,
}

impl KeyCard {
  pub fn new(access_level: u16) -> Self {
    Self { access_level }
  }

  pub fn get_access_level(&self) -> u16 {
    self.access_level
  }
}

#[derive(Debug)]
pub struct Database;

impl Database {
  pub fn connect() -> Result<Self, String> {
    Ok(Self)
  }

  pub fn find_employee(&self, name: &str) -> Result<Employee, String> {
    match name {
      | "Logno" | "Atoshi" | "SKN" => Ok(Employee::new(name.to_owned())),
      | _ => Err("Employee Not Found!".to_owned()),
    }
  }

  pub fn get_keycard_level(&self, employee: &Employee) -> Result<KeyCard, String> {
    //* For Borrowing A Struct, you must use `.as_str()` when it's a matter of String
    match employee.name.as_str() {
      | "Logno" => Ok(KeyCard::new(1000)),
      | "Atoshi" => Ok(KeyCard::new(800)),
      | "SKN" => Ok(KeyCard::new(500)),
      | _ => Err("Employee do not have a Key Card!".to_owned()),
    }
  }
}
