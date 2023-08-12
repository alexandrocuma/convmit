use clap::Parser;
use console::Style;

mod cli_struct;
mod command;
mod configuration;
mod git_actions;
mod interactive;
mod shared;

use crate::configuration::create_configuration;
use git_actions::commit;

fn main() {
    let cyan = Style::new().cyan();
    let cli = cli_struct::Cli::parse();

    match &cli.command {
        Some(cli_struct::Commands::Init) => {
            create_configuration(".convmit/");
        }
        Some(cli_struct::Commands::Gen(args)) => {
            let conventional_commit = command::cli(args);
            commit::commit(&conventional_commit);
            println!(
                "Changes commited with the message:\n\n{}\n",
                cyan.apply_to(&conventional_commit)
            );
        }
        None => {
            let conventional_commit = interactive::cli();
            commit::commit(&conventional_commit);
            println!(
                "Changes commited with the message:\n\n{}\n",
                cyan.apply_to(&conventional_commit)
            );
        }
    }
}
