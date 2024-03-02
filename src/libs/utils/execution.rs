use console::style;
use std::{
  io::Error,
  process::{Child, Command},
};

pub fn execute_command(program: &str, args: &[&str]) {
  let result: Result<Child, Error> = Command::new(program).args(args).spawn();

  match result {
    | Ok(_) => (),
    | Err(e) => {
      print!("Error: {} \n", format!("{}", style(e).red()));
    },
  }
}

pub fn gt_execute_command(arg: &str) {
  // Command::new("gnome-terminal")
  //   .args(["--", "bash", "-c", arg])
  //   .spawn()
  //   .expect("Command couldn't be executed! 👎");
  let result: Result<Child, Error> = Command::new("gnome-terminal")
    .args(["--", "bash", "-c", arg])
    .spawn();

  match result {
    | Ok(_) => (),
    | Err(e) => print!("Error: {} \n", format!("{}", style(e).red())),
  }
}
