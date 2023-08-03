use clap::Parser;
use console::Style;
use clipboard::{ClipboardContext, ClipboardProvider};

mod interactive;
mod command;

fn main() {
  let args = command::Cli::parse();
  
  let cyan = Style::new().cyan();
  let green = Style::new().green();

  let conventional_commit = if args.commit_type.interactive {
    interactive::cli()
  } else {
    command::cli(args)
  };
  
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  ctx.set_contents(conventional_commit.to_string()).unwrap();

  println!("\n");
  println!("{}", cyan.apply_to("The commit message has been copied to your clipboard!"));
  println!("Your message is: {}", green.apply_to(conventional_commit));
}