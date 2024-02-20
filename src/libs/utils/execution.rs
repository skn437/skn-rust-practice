use std::process::Command;

pub fn execute_command(program: &str, args: &[&str]) {
  Command::new(program)
    .args(args)
    .spawn()
    .expect("Command couldn't be executed!");
}

pub fn gt_execute_command(arg: &str) {
  Command::new("gnome-terminal")
    .args(["--", "bash", "-c", arg])
    .spawn()
    .expect("Command couldn't be executed! 👎");
}
