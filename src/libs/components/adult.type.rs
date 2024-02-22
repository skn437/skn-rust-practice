pub struct Adult {
  name: String,
  age: u8,
}

impl Adult {
  pub fn new(name: &str, age: u8) -> Result<Self, &str> {
    if age >= 18 {
      Ok(Self {
        name: name.to_owned(),
        age,
      })
    } else {
      Err("Age must be at least 18")
    }
  }

  //* This is how an instance status (if it's ok or an error) should be checked!
  pub fn check_status(adult: Result<Self, &str>) {
    match adult {
      | Ok(instance) => print!("{} is {} years old. \n", instance.name, instance.age),
      | Err(e) => print!("Error: {} \n", e),
    }
  }
}
