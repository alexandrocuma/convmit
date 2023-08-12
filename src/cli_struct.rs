use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Gen(GenArgs),
}

#[derive(Debug, Args)]
pub struct GenArgs {
    #[command(flatten)]
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

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]

pub struct CommitTypes {
    #[arg(long)]
    pub fix: bool,

    #[arg(long)]
    pub feat: bool,

    #[arg(long)]
    pub docs: bool,

    #[arg(long)]
    pub style: bool,

    #[arg(long)]
    pub refactor: bool,

    #[arg(long)]
    pub perf: bool,

    #[arg(long)]
    pub test: bool,

    #[arg(long)]
    pub chore: bool,
}
