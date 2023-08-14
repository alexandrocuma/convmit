use clap::Subcommand;

use crate::command_line::gen_args::GenArgs;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Gen(GenArgs),
}