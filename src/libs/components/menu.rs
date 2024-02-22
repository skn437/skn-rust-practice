use super::menu_type::Menu;

fn get_choice(input: &str) -> Result<Menu, String> {
  match input {
    | "menu" => Ok(Menu::MenuItem),
    | "start" => Ok(Menu::Start),
    | "quit" => Ok(Menu::Quit),
    | _ => Err("Invalid Input!".to_owned()),
  }
}

fn print_choice(choice: &Menu) {
  print!("Your Choice: {:?} \n", choice);
}

pub fn pick_choice(input: &str) -> Result<(), String> {
  let choice: Menu = get_choice(input)?; //* `?` tells the code not to continue if it returns `Err()`

  print_choice(&choice);

  Ok(())
}
