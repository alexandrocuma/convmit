use clap::Args;

use crate::cli_structure::commit_types::CommitTypes;

#[derive(Debug, Args)]
pub struct GenArgs {
    #[command(subcommand)]
    pub commit_type: CommitTypes,

    #[arg(short, long)]
    pub scope: Option<String>,

    #[arg(short = 'b', long)]
    pub breaking_change: bool,

    #[arg(short, long, required = true)]
    pub description: Option<String>,

    #[arg(short = 'o', long)]
    pub body: Option<String>,

    #[arg(short = 't', long)]
    pub footer: Option<String>,
}