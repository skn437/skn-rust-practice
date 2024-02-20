use console::style;

pub struct User {
  name: String,
  age: Option<u8>, //* `Option` is predefined enum with variants: `Some() & None`
  email: String,
}

impl User {
  pub fn new(name: String, age: Option<u8>, email: String) -> Self {
    Self { name, age, email }
  }

  pub fn show_age(&self) {
    match self.age {
      | Some(age) => print!("{}'s age is: {} & email: {} \n", self.name, age, self.email),
      | None => print!(
        "{}{} \n",
        self.name,
        style("'s age is not provided!").blue(),
      ),
    }
  }
}
