//? Modules
mod libs;
mod tests;

//? Usage::Source
//use libs::functions::option::Option;
//use libs::utils::Direction;
use libs::{
  components::user::create_user, components::user_type, functions::option::Option, utils::Direction,
};

fn main() {
  println!("Hello, SKN!");

  //* Variable Declaration
  // let two: i8 = 2;
  // let name: &str = "SKN";
  // let skn: char = 's';
  // let mut title: &str = "Wang";
  // let quit_program: bool = false;

  //* Function Creation
  fn sum(num1: i8, num2: i8) -> i8 {
    num1 + num2 //* short hand return doesn't need `;` to end statement
  }

  //* println! Macro
  //* Macros use ! mark to get executed
  //* `{:?}` is called debug token, `{}` is normal token
  println!("{:?} 👍 {:?}", sum(4, 3), sum(5, 2)); //* Sequence of token {:?}, {}, {} and so on
  print!("{:?} 👍 {:?} \n", sum(4, 3), sum(5, 2));
  println!("{:?} 👍 {:?}", sum(4, 3), sum(5, 2));
  print!("{} \n", sum(6, 1));

  let num: u8 = 7; //* `let` is a binding

  if num < 7 {
    print!("Hello! \n");
  } else {
    print!("Hello Wang So! \n");
  }

  let mut i: u8 = 0;

  //* `loop` is infinite loop
  loop {
    if i > 5 {
      break;
    }

    print!("{} \n", i);
    i += 1;
  }

  //* `match` is just like `switch` in Java & TypeScript
  fn moniker(number: u8) {
    match number {
      //* Instead of Java & TypeScript Case, in Rust it is `|`
      | 1 => {
        print!("Hello SKN! \n");
        print!("This is 1 \n");
      },
      | 2 => {
        print!("Hello Wang So! \n");
        print!("This is 2 \n");
      },
      | 3 => print!("This is 3 \n"),
      | _ => print!("This is some other value. \n"),
    }
  }

  moniker(1);
  moniker(2);
  moniker(3);
  moniker(7);

  fn get_direction(direction: Direction) -> u8 {
    match direction {
      | Direction::Up => 1,
      | Direction::Right => 2,
      | Direction::Bottom => 3,
      | Direction::Left => 4,
    }
  }

  //? Never Print any field of enum wth print! or println! macros
  //* If all fields of an enum is not used then the rust analyzer throws warning
  get_direction(Direction::Up); //* `::` is just like Java & TypeScript `.`
  get_direction(Direction::Right);
  get_direction(Direction::Bottom);
  get_direction(Direction::Left);

  //* `struct` is like TypeScript `interface`
  struct GroceryItemType {
    stock: i32,
    price: f64,
  }

  let pizza: GroceryItemType = GroceryItemType {
    stock: 32,
    price: 12.3,
  };

  print!("Pizza Stock: {} \n", pizza.stock);
  print!("Pizza Price Per Unit: {} \n", pizza.price);

  // enum AccessType {
  //   Full,
  // }

  fn get_tuple() -> (i8, f64, bool) {
    (7, 14.7, true)
  }

  let (x, y, z) = get_tuple();

  print!("x: {}, y: {}, z: {} \n", x, y, z);

  //* Don't put any `enum` field in a single named tuple, instead destructure
  //* e.g. let tuple: (u8, AccessType) = (7, AccessType::Full); Don't use like this
  //* instead use this: let (number, access): (u8, AccessType) = (7, AccessType);
  // let (num7, access): (u8, AccessType) = (7, AccessType::Full);

  let tuple: (i8, i8) = (7, 8);

  // print!("{}", AccessType::Full);
  // print!("Num7: {}, Access: {} \n", num7, AccessType::Full);
  print!("Element 1: {} \n", tuple.0); //* Single named tuple can be destructures through `.` and index number

  //* Rust is expression based language
  //* most things are evaluated and return some value
  //* Things can be nested very easily for that

  //* Each item of an enum is called a variant
  enum AccessLevelType {
    Admin,
    Manager,
    Guest,
  }

  impl AccessLevelType {
    fn print(&self) {
      match self {
        | Self::Admin => {
          print!("admin \n");
        },
        | Self::Guest => {
          print!("guest \n");
        },
        | Self::Manager => {
          print!("manager \n");
        },
      }
    }
  }

  let access1: AccessLevelType = AccessLevelType::Guest;
  access1.print();

  let access2: AccessLevelType = AccessLevelType::Admin;
  let access3: AccessLevelType = AccessLevelType::Manager;

  let has_access: bool = match access1 {
    | AccessLevelType::Admin => true,
    | _ => false,
  };

  print!("Access: {} \n", has_access);

  fn show_access(access: AccessLevelType) {
    match access {
      | AccessLevelType::Admin => print!("Admin \n"),
      | AccessLevelType::Guest => print!("Guest \n"),
      | AccessLevelType::Manager => print!("Manager \n"),
    }
  }

  show_access(access2);
  show_access(access3);

  // show_access(access2); //* This statement won't work as the ownership has been moved to `show_access`
  /*
   * In Rust, when a variable is declared in a specific function, that function becomes the owner & only that has the right to delete the variable after reaching to the end
   * Ownership Borrowing Data gives performance boost rather than Ownership Moving
   */

  struct User {
    id: u8,
    admin: bool,
  }

  //* `struct` & `enum` can have `impl` as lists of methods
  impl User {
    //* When using `&self`, it will work like Java Object Methods
    fn show_id(&self) {
      print!("Id is: {} \n", self.id);
    }

    //* Without parameters
    fn show_admin(&self) {
      print!("Admin: {} \n", self.admin);
    }

    //* With parameters
    fn show_count(&self, number: u8) {
      print!("Count: {} \n", number);
    }

    //* When using `Self`, it will work like Java Static Methods
    //* It's a good practice to name a static method `new` when constructing or returning `Self`
    fn new(id: u8, admin: bool) -> Self {
      Self { id, admin }
    }
  }

  let user: User = User {
    id: 77,
    admin: false,
  };

  user.show_id();
  user.show_admin();

  //* `Self` implemented functions (Java Static Methods) are called like this
  let admin: User = User::new(7, true);

  admin.show_id();
  admin.show_admin();
  admin.show_count(67);

  //* Vector
  //* It's just like TypeScript Array or Java List

  //* Vector Creation Way: 1
  //* vec! macro
  let vector1: Vec<u8> = vec![1, 2, 3];

  print!("Vector1: {:?} \n", vector1); //* To print a vector, you need to use debig token i.e. {:?}

  //* Vector Creation Way: 2
  //* Vec::new()
  let mut vector2: Vec<u8> = Vec::new();

  vector2.push(1);
  vector2.push(2);
  vector2.push(3);
  vector2.pop();

  print!("Vector 2: {:?} \n", vector2);
  print!("Vector 2 Length: {} \n", vector2.len());
  print!("Vector2 1st Element: {} \n", vector2[0]);

  let mut vector3: Vec<u8> = vec![6, 7];

  vector3.push(8);

  for number in vector3 {
    print!("Element: {} \n", number);
  }

  let mut vector4: Vec<u8> = vec![];

  vector4.push(7);

  print!("Vector 4: {:?} \n", vector4);

  struct Color {
    id: u8,
    count: u8,
  }

  impl Color {
    fn new(id: u8, count: u8) -> Self {
      Self { id, count }
    }
  }

  //* It's just like TypeScript Array of Color Object
  let vector5: Vec<Color> = vec![Color::new(1, 2), Color::new(3, 4), Color::new(6, 7)];

  for item in vector5 {
    print!("Item ID: {} \n", item.id);
    print!("Item Count: {} \n", item.count);
  }

  // print!("Length of Vector5: {} \n", vector5.len()); //* This won't work as vector5 ownership has been moved to the for loop & got destroyed after completion

  //* String
  /*
    * (1) `String` : it is owned i.e. it should only be used in a struct, a enum because structs cleans up memories on their own and with the borrowed string it is not possible
    * (2) `&str` : it is borrowed i.e. it should only be used in functions
    * (3) Strings are automatically borrowed, so careful when using strings; infact structs must not have borrowed variables
    * (4) To make a string not borrowed, you can use either `.to_owned()` or `String::from()` methods

  */

  let option: Option = Option::new(7, String::from("google"), 1);

  option.format();

  //* Derive
  //* To print out `enum` & `struct`, you must use derive.
  //* `#[]` is called `attribute` in Rust

  #[derive(Debug, Clone, Copy)] //* Clone & Copy ensures ownership intact, instead it clones and copies
  enum Person {
    Best,
    Bullshit,
  }

  //* If debugging a struct that contains an enum, then that enum must be debugged with derive as well
  #[derive(Debug, Clone)]
  struct Human {
    name: String,
    character: Person,
  }

  impl Human {
    fn new(name: String, character: Person) -> Self {
      Self { name, character }
    }
  }

  let human: Human = Human::new(String::from("SKN"), Person::Best);

  print!("{:?}", human);

  //* Advanced Enums Variants With Values
  enum Discount {
    Flat(u8),
    Percent(u8),
  }

  struct Ticket {
    event: String,
    price: f32,
  }

  impl Ticket {
    fn new(event: String, price: f32) -> Self {
      Self { event, price }
    }
  }

  let flat: Discount = Discount::Flat(7);

  //* Advanced `match`
  match flat {
    | Discount::Flat(7) => print!("Flat Disount: 7 \n"),
    | Discount::Flat(other) => print!("Flat Discount With Other Value: {} \n", other),
    | Discount::Percent(_) => (), //* `_` are ignored, so it is better to use `other` to handle all others
  }

  let ticket: Ticket = Ticket::new(String::from("Fair!"), 30.0);

  match ticket {
    | Ticket { price, .. } => print!("Ticket Price: {} \n", price), //* In `struct`, `..` means any other/  ignore all other
  }

  enum AnimeTicket {
    Standard(f32),
    VIP(f32, String),
    BackSeat(f32, String),
  }

  let tickets: Vec<AnimeTicket> = vec![
    AnimeTicket::Standard(10.0),
    AnimeTicket::VIP(30.0, String::from("Wang So")),
    AnimeTicket::BackSeat(15.0, String::from("Shukhan")),
  ];

  for ticket in tickets {
    match ticket {
      | AnimeTicket::BackSeat(price, holder) => print!("Price @{}, Holder: {} \n", price, holder),
      | AnimeTicket::VIP(price, holder) => print!("Price @{}, Holder: {} \n", price, holder),
      | AnimeTicket::Standard(price) => print!("Price @{} \n", price),
    }
  }

  let user: user_type::User =
    create_user("Wang".to_owned(), None, "skn437physx@gmail.com".to_owned());

  user.show_age();
}
