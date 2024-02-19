pub struct Option {
  id: u8,
  title: String,
  count: u8,
}

impl Option {
  pub fn new(id: u8, title: String, count: u8) -> Self {
    Self { id, title, count }
  }

  pub fn format(&self) {
    print!(
      "Id: {}, Title: {}, Count: {} \n",
      self.id, self.title, self.count
    );
  }
}
