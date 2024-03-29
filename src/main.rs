//? Modules
mod libs;
mod tests;

use std::{
  collections::HashMap,
  fmt::Display,
  io::{self, ErrorKind, Read},
};

//? Usage::Source
//use libs::functions::option::Option;
//use libs::utils::Direction;
use libs::{
  components::access::authorize, components::access_type::AuthorizationStatus,
  components::access_type::ProtectedLocation, components::adult_type::Adult,
  components::grocery_type::Grocery, components::menu, components::random::gen_random_number,
  components::random::get_guess, components::selection, components::stock_type::stock_info,
  components::user::create_user, components::user_type, functions::option,
  utils::env::ExternalConfig, utils::execution::execute_command,
  utils::execution::gt_execute_command, utils::message, utils::stdio::get_reader_input,
  utils::Direction,
};
use std::fs::File;
use unicode_segmentation::UnicodeSegmentation;

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
  //* Functions invoked with "::" is called `Associative Functions`
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

  //* Tuple
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

  let option: option::Option = option::Option::new(7, String::from("google"), 1);

  option.format();

  //* Derive
  //* To print out `enum` & `struct`, you must use derive.
  //* `#[]` is called `attribute` in Rust

  #[derive(Debug, Clone, Copy)] //* Clone & Copy ensures ownership intact, instead it clones and copies
  #[allow(dead_code)]
  enum Person {
    Best,
    Bullshit,
  }

  //* If debugging a struct that contains an enum, then that enum must be debugged with derive as well
  #[derive(Debug, Clone)]
  #[allow(dead_code)]
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
  #[allow(dead_code)]
  enum Discount {
    Flat(u8),
    Percent(u8),
  }

  #[allow(dead_code)]
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

  let cereal: Grocery = Grocery::new("cereal".to_owned(), 5);
  let egg: Grocery = Grocery::new("egg".to_owned(), 9);
  let noodles: Grocery = Grocery::new("noodles".to_owned(), 7);

  let grocery_items: Vec<Grocery> = vec![cereal, egg, noodles];

  let item_count: Option<u8> = Grocery::check_quantity(grocery_items, "egg");

  print!("Item Count: {:?} \n", item_count);

  //* Execute Commands
  execute_command(
    "gnome-terminal",
    &["--", "bash", "-c", "bash cmd.sh; read -n 1 KEY"],
  );

  gt_execute_command("bash cmd.sh; read -n 1 KEY");

  //* Result<>, Ok(), Err() Practice
  //* If any function returns `Result<>`, then it should be caught in a variable as error may occur
  //* `main()` function should not carry `?` operator to handle error
  let choice: Result<(), String> = menu::pick_choice("stat");
  print!("Result: {:?}\n", choice);

  let select: Result<(), String> = selection::pick_choice("docker");
  print!("Selection Result: {:?} \n", select);

  let adult: Result<Adult, &str> = Adult::new("SKN", 15);

  Adult::check_status(adult);

  let adult2: Result<Adult, &str> = Adult::new("Wang So", 30);
  Adult::check_status(adult2);

  let logno: Result<AuthorizationStatus, String> = authorize("Logno", ProtectedLocation::All);
  print!("Logno: {:?} \n", logno);

  let atoshi: Result<AuthorizationStatus, String> =
    authorize("Atoshi", ProtectedLocation::Warehouse);
  print!("Atoshi: {:?} \n", atoshi);

  let skn: Result<AuthorizationStatus, String> = authorize("SKN", ProtectedLocation::Office);
  print!("SKN: {:?} \n", skn);

  let wang: Result<AuthorizationStatus, String> =
    authorize("Wang So", ProtectedLocation::Warehouse);
  print!("Wang: {:?} \n", wang);

  //* HashMap Practice
  //* HashMap can retrieve data very fast
  //* Data is printed in random order just like `Java`, so be careful!
  stock_info();

  //* User Input
  let user_input: String = get_reader_input("Your Name");
  print!("User Input: {} \n", user_input);

  print!("{} \n", message::Color::blue("Hello Fourth Prince!"));

  //* */
  //* LET'S GET RUSTY
  //* LET'S GET RUSTY
  //* LET'S GET RUSTY
  //* */
  //* Random Number
  print!("Random Number: {} \n", gen_random_number(1, 100));

  get_guess();

  //* Constant & Number Separator
  const PI: f32 = 3.1416;
  let num_num: i32 = 100_000_000;

  print!("PI: {}, Num: {} \n", PI, num_num);

  //* Exclusive Range
  //* 1..4 means from 1 to 3
  for number in 1..4 {
    print!("{} \n", number);
  }

  //* Inclusive Range
  //* 1..=4 means from 1 to 4
  for number in 1..=4 {
    print!("{} \n", number);
  }

  //* Ownership
  let s1: &str = "SKN";
  let s2: &str = s1; //* This is ok as string slice (immutable) is borrowed */
  let s3: String = String::from("SKN");
  let s4: String = s3.clone(); //* You should not do that with `String`, as s3 will get destroyed as the ownership has been passed

  print!("s1: {} \n", s1);
  print!("s2: {} \n", s2);
  print!("s3: {} \n", s3);
  print!("s4: {} \n", s4);

  //* Int
  let num100: i32 = 45;
  let num101: i32 = num100;

  print!("num100: {} \n", num100);
  print!("num101: {} \n", num101);

  //*  Rust automatically copies: int, float, char, boolean and tuple(not having String value) types
  //* They are automatically borrowed
  //* Borrowed Values are immutable
  //* To mutate a borrowed variable, a mutable reference should be passed

  let mut str1: String = String::from("Hello SKN");

  fn change_string(text: &mut String) -> String {
    text.push_str("!");
    text.to_string() //*  .to_string() can convert &mut String into String
  }

  print!("{} \n", change_string(&mut str1));

  //* To just borrow a String:
  let str200: String = String::from("Hello Wang!");

  fn borrow_string(text: &String) -> String {
    text.to_string() //*  .to_string() can convert &String into String
  }

  let str201: String = borrow_string(&str200);

  print!("str200: {} \n", str200);
  print!("str201: {} \n", str201);

  //* You can't have more than one &mut reference of the same variable in the same scope, only one is allowed
  //* You can't have a mix of mutable reference (&mut) and immutable reference(&) of the same variable in the same scope
  //* You can have more than one immutable reference(&) of the same variable in the same scope
  //* A function should not return a referenced value

  //* Config
  let config: ExternalConfig = ExternalConfig::new();

  print!("Author Name: {:?} \n", config.author.name);
  print!("Author Age: {:?} \n", config.author.age);

  //* String Slice/ String Literal
  let string_name: String = String::from("Hello Wang So!");

  //* String Slice Can be sliced like this
  let last_name: &str = &string_name[6..10];
  let first_name: &str = &string_name[11..13];

  print!("My last name is {} \n", last_name);
  print!("My first name is {} \n", first_name);

  //* get first name
  fn get_first_name(name: &str) -> String {
    for (index, char) in name.trim().char_indices() {
      if char == ' ' {
        return name[..index].to_string();
      }
    }

    name[..].to_string()
  }

  //* get last name
  fn get_last_name(name: &str) -> String {
    for (index, char) in name.trim().char_indices() {
      if char == ' ' {
        return name[index + 1..].to_string();
      }
    }

    String::from("")
  }

  print!(
    "My name is Wang So! My first name is {} \n",
    get_first_name("Wang So")
  );

  print!(
    "My name is Wang So! My last name is {} \n",
    get_last_name("Wang So")
  );

  //* Array
  let arr1: [i8; 5] = [1, 2, 3, 4, 5];
  let arr2: &[i8] = &arr1[1..4]; //* This has to be borrowed or else it won't work

  print!("Sliced Array: {:?} \n", arr2);

  //* `..` operator comes in various flavors. Not only in "slicing", but also in like TypeScript `...` operator
  //* Mainly in struct new instance creation
  struct Hello {
    id: u8,
    name: String,
  }

  impl Hello {
    fn new(id: u8, name: &str) -> Self {
      Self {
        id,
        name: name.to_string(),
      }
    }
  }

  let hello1: Hello = Hello::new(1, "Wang");
  let hello2: Hello = Hello {
    id: 2,
    ..hello1 //* Don't use comma when using `..` operator. The problem is in this way ownership is moved!! Not good way to do things
  };

  print!("hello1 id: {}, hello2 name: {} \n", hello1.id, hello2.name);

  #[derive(Debug)]
  struct Rectangle {
    width: u32,
    height: u32,
  }

  //* Multiple `impl` can be written
  impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
      Self { width, height }
    }
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
      self.width > other_rectangle.width && self.height > other_rectangle.height
    }
  }

  let rectangle1: Rectangle = Rectangle::new(120, 60);
  let rectangle2: Rectangle = Rectangle::new(60, 40);

  print!("Rectangle2: {:#?} \n", rectangle2);

  print!("Area of Rectangle1: {:#?} \n", rectangle1.area());
  print!(
    "Rectangle 1 can hold Rectangle 2: {} \n",
    rectangle1.can_hold(&rectangle2)
  );

  //* Enum variant data
  #[derive(Debug)]
  #[allow(dead_code)]
  enum Example {
    NoData,
    StructData { x: i32, y: f64, z: String },
    SingleData(String),
    MultipleData(i32, String),
  }

  //* To handle `null` values, use `Option<>`

  #[derive(Debug)]
  enum Test {
    Java(String),
    TS,
  }

  fn test_lang(test: &Test) -> u8 {
    //* Each checking in `match` is called an arm
    match test {
      | Test::TS => {
        print!("Hello TS! \n");
        7
      },
      | Test::Java(value) => {
        print!("Java Value: {} \n", value);
        0
      },
    }
  }

  test_lang(&Test::TS);
  test_lang(&Test::Java(String::from("77")));

  let test1: Test = Test::TS;
  test_lang(&test1);
  print!("Test1: {:?} \n", test1);

  //* Vector
  //* The vector has no size limit
  //* It is better to use vector.get() to access an element and handle it with `match` rather than using vector[index] because it will cause runtime error
  let vector1: Vec<i8> = vec![1, 2, 3, 4];

  //* vector.get() returns `Option()`, so it needs to be handled by `Some()` & `None`
  match &vector1.get(4) {
    | Some(value) => print!("Vector1 4th Element: {} \n", value),
    | None => print!("Index out of bound! \n"),
  }

  //* Always use reference in `for`, `match` & `function`
  for item in &vector1 {
    print!("Vector1 Item: {} \n", item);
  }

  let mut vector2: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7];

  for item in &mut vector2 {
    *item += 3 //* You need to use dereference `*` operator to change mutable variable
  }

  print!("Vector2: {:?} \n", vector2);

  let num7: i8 = 69;

  fn print_num(num: &i8) -> i8 {
    *num //* You need to dereference to get the value
  }

  print!("Print Num: {} \n", print_num(&num7));

  //* Vector cannot contain different types of data. But to do that you can use `enum`
  #[derive(Debug)]
  enum Data {
    Int(i8),
    Float(f64),
    String(String),
  }

  let vector: Vec<Data> = vec![
    Data::Int(7),
    Data::Float(14.4),
    Data::String(String::from("Wang So!")),
  ];

  print!("Vector of different data: {:?} \n", vector);

  match &vector[1] {
    | Data::Float(value) => print!("The value of float: {} \n", value),
    | _ => print!("Not a float \n"),
  }

  print!("Float: {:?} \n", vector[1]);

  //* Strings & Characters
  let mut user: String = String::from("Wang");
  user.push_str(" So"); //* String's .push_str() takes a string literal/string slice (&str)
  user.push('!'); //* String's .push() takes a character

  print!("Hey there, {} \n", user);
  //* format!, print! and other macros don't take ownerships

  let my_name: String = String::from("Wang So");

  fn string_to_slice(text: &str) -> String {
    text[5..].to_string()
  }
  let first_name: String = string_to_slice(&my_name);

  print!("First Name: {} \n", first_name);

  //* When using String's .bytes(), .chars() or .graphemes(), you don't need to borrow or use reference operator
  for byte in my_name.bytes() {
    print!("Bytes of my name: {} \n", byte);
  }

  let bengali_name: String = String::from("সুখন");

  for grapheme in bengali_name.graphemes(true) {
    print!("Grapheme of my name: {} \n", grapheme);
  }

  print!("My Bengali Name: {} \n", bengali_name);

  let sanskrit_greeting: String = String::from("नमस्ते");

  for grapheme in sanskrit_greeting.graphemes(true) {
    print!("Grapheme of my another name: {} \n", grapheme);
  }

  print!("Sanskrit Greeting: {} \n", sanskrit_greeting);

  print!("my name: {} \n", my_name);

  //* Hash Map
  //* Even hash maps can get ownerships, so be careful again
  let blue: String = String::from("blue");
  let green: String = String::from("green");

  let mut hashmap7: HashMap<String, u8> = HashMap::new();

  hashmap7.insert(blue, 7); //* Here ownership of `blue` has been moved. It cannot be used again.
  hashmap7.insert(green, 77);

  //* .entry().or_insert() returns the value of hash map with the given key
  hashmap7.entry(String::from("blue")).or_insert(78); //* This won't work as "blue" key is already present
  hashmap7.entry(String::from("Blue")).or_insert(78);

  print!("Hash Map: {:#?} \n", hashmap7);

  let mut new_map: HashMap<String, u8> = HashMap::new();

  let key_string: &str = "hello skn good skn";

  //* .split_whitespace() is used to split a string by whitespace character
  for word in key_string.split_whitespace() {
    let data: &mut u8 = new_map.entry(word.to_string()).or_insert(0);
    *data += 1; //* `&mut` needs dereference `*` operator to get & mutate the value
  }
  print!("New Map: {:?} \n", new_map);

  //* File Creation
  fn open_file() {
    let result: Result<File, std::io::Error> = File::open("hello.txt");

    match result {
      | Ok(value) => {
        print!("{:?} \n", value);
      },
      //* e.kind() can give the enum of all error kinds
      | Err(e) => match e.kind() {
        | ErrorKind::NotFound => {
          print!("File Not Found! ❌ \n");
          print!("Attempting to create the file... \n");
          create_file();
        },
        | _ => panic!("Unknown Error! \n"),
      },
    }
  }

  fn create_file() {
    let result: Result<File, io::Error> = File::create("hello.txt");

    match result {
      | Ok(_) => print!("File Created Successfully! ✅ \n"),
      | Err(e) => print!("{:?}: Couldn't create the file! ❌ \n", e), //* if debug token is used then you can see all the contents of `error`
    }
  }

  fn read_file() -> Result<String, io::Error> {
    let result: Result<File, io::Error> = File::open("hello.txt");

    //* Shadowing `result`
    //* Good practice to use in error handling
    let mut result: File = match result {
      | Ok(value) => value,
      | Err(e) => return Err(e),
    };

    let mut s: String = String::new();

    match result.read_to_string(&mut s) {
      | Ok(_) => Ok(s),
      | Err(e) => Err(e),
    }
  }

  #[allow(dead_code)]
  fn create_file_example() -> Result<File, io::Error> {
    //* `?` returns error if error occurs and ends the function without going to the next line
    //* `?` should be used in a custom function rather than in `main` function
    let result: File = File::create("skn.txt")?;

    //* .unwrap() panics if error occurs. Not recommended way to handle errors
    let _: File = File::create("skn.txt").unwrap();

    //* .expect() panics if error occurs but with custom message. Not recommended way to handle errors
    let _: File = File::create("skn.txt").expect("Couldn't create the file! \n");

    Ok(result)
  }

  open_file();

  let res: Result<String, io::Error> = read_file();

  match res {
    | Ok(value) => print!("File Content: \n{}\n", value),
    | Err(e) => print!("Error: {} \n", e),
  }

  #[derive(Debug)]
  #[allow(dead_code)]
  struct Point<T, S> {
    x: T,
    y: S,
  }

  //* if `enum` or `struct` has generic type then their corresponding `impl` must contain the same generics
  impl<T, S> Point<T, S> {
    fn new(x: T, y: S) -> Self {
      Self { x, y }
    }
  }

  //* Separate `impl` can be created for separate types
  //* This method will only available if x is f64 type
  #[allow(dead_code)]
  impl<S> Point<f64, S> {
    fn float_only(&self) -> f64 {
      self.x
    }
  }

  print!("Point: {:?} \n", Point::new(12.0, 'c'));

  //* Traits
  #[allow(dead_code)]
  struct Tweet {
    username: String,
    content: String,
    reply: String,
    email: String,
  }

  //* `Trait` is just like Java `Interface` & `Abstract Class` Mixture
  trait Summary {
    fn summarize(&self) -> String;

    //* default trait mathods which are defined inside a `trait`, can be used as it is or can be redefined
    fn welcome(&self) {
      print!("Hello!!! \n");
    }
  }

  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{} {}", self.username, self.email)
    }
  }

  let t1: Tweet = Tweet {
    username: String::from("skn437"),
    content: String::from("Hello SKN!"),
    reply: String::from("Nothing"),
    email: String::from("skn437physx@gmail.com"),
  };

  print!("Tweet1 Summary: {} \n", t1.summarize());
  t1.welcome();

  fn notify(item: &impl Summary) {
    print!("Notification: {} \n", item.summarize());
  }

  //* Multi `trait` impl can be done by using `+`
  #[allow(dead_code)]
  fn notify2(item: &(impl Summary + Display)) {
    print!("Notification: {} \n", item.summarize());
  }

  notify(&t1);

  //* `where` keyword to use in Generics for `trait` only
  #[allow(dead_code)]
  fn notify3<T, S>(item1: &T, item2: &S) -> i32
  where
    T: Summary + Display,
    S: Summary,
  {
    print!("{} {} \n", item1.summarize(), item2.summarize());

    let num: i32 = 7;
    num
  }
}
