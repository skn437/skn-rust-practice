use super::selection_type::Selection;

fn get_choice(input: &str) -> Result<Selection, String> {
  match input {
    | "spring" => Ok(Selection::SpringBoot),
    | "svelte" => Ok(Selection::SvelteKit),
    | "docker" => Ok(Selection::Docker),
    | _ => Err("Invalid Choice".to_owned()),
  }
}

fn print_choice(choice: Selection) {
  print!("Your Selection: {:?} \n", choice);
}

pub fn pick_choice(input: &str) -> Result<(), String> {
  let choice: Selection = get_choice(input)?;

  print_choice(choice);

  Ok(())
}
