use super::user_type::User;

pub fn create_user(name: String, age: Option<u8>, email: String) -> User {
  let user: User = User::new(name, age, email);

  return user;
}
