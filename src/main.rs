mod configuration;
mod cli_structure;
mod command_line;
mod git_actions;
mod coauthors;
mod steps;

use clap::Parser;
use console::Style;
use git_actions::commit;
use cli_structure::{interface::Cli, commands::Commands};

use crate::steps::Steps;

fn main() {
  let cyan = Style::new().cyan();
  let cli = Cli::parse();

  match &cli.command {
      Some(Commands::Init) => {
          configuration::create_configuration(".convmit/");
      }
      Some(Commands::Gen(args)) => {
          let mut commit = Steps::new();
          let commit_message = &commit.generate_command(args);
          commit::commit(&commit_message);
          println!(
              "Changes commited with the message:\n\n{}\n",
              cyan.apply_to(&commit_message)
          );
      }
      None => {
          let mut commit = Steps::new();
          let commit_message = &&commit.generate_interactive();
          commit::commit(&commit_message);
          println!(
              "Changes commited with the message:\n\n{}\n",
              cyan.apply_to(&commit_message)
          );
      }
  }
}
