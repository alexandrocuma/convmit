use clap::Parser;
use console::Style;
use dialoguer::Confirm;
use clipboard::{ClipboardContext, ClipboardProvider};

mod git_actions;
mod interactive;
mod command;

use git_actions::{add, commit};

fn main() {
  let args = command::Cli::parse();
  let green = Style::new().green();

  git_add();
  
  let conventional_commit = if args.commit_type.interactive {
    interactive::cli()
  } else {
    command::cli(args)
  };

  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  ctx.set_contents(conventional_commit.to_string()).unwrap();

  println!("\n");
  commit::commit(&conventional_commit);
  println!("Changes commited with the message: {}", green.apply_to(&conventional_commit));

}

fn git_add() {
  let green = Style::new().green();

  if Confirm::new().with_prompt("Add all files to commit ?").interact().unwrap() {
    add::add_all();
    println!("{}", green.apply_to("Added all files into the new commit"));
  }
}
