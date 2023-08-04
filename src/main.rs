use clap::Parser;
use console::Style;
use dialoguer::Confirm;

mod git_actions;
mod interactive;
mod command;

use git_actions::{add, commit, push};

fn main() {
  let cyan = Style::new().cyan();
  let green = Style::new().green();
  
  let args = command::Cli::parse();

  git_add(&green);
  
  let conventional_commit = if args.commit_type.interactive {
    interactive::cli()
  } else {
    command::cli(args)
  };

  println!("\n");
  commit::commit(&conventional_commit);
  println!("Changes commited with the message: {}", cyan.apply_to(&conventional_commit));

  git_push(&green)
}

fn git_add(color: &Style) {
  if Confirm::new().with_prompt("Add all files to commit?").interact().unwrap() {
    add::add_all();
    println!("{}", color.apply_to("Added all files into the new commit"));
  }
}

fn git_push(color: &Style) {
  if Confirm::new().with_prompt("Do you want to push all your changes?").interact().unwrap() {
    push::push();
    println!("{}", color.apply_to("Changes pushed into the branch"));
  }
}