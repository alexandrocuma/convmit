use clap::{Args, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(flatten)]
  pub commit_type: CommitTypes,

  #[arg(short, long)]
  pub scope: Option<String>,

  #[arg(short = 'i', long)]
  pub breaking_change: bool,

  #[arg(short, long)]
  pub description: Option<String>,

  #[arg(short = 'o', long)]
  pub body: Option<String>,

  #[arg(short = 't', long)]
  pub footer: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct CommitTypes {
  #[arg(short = 'x', long)]
  pub fix: bool,

  #[arg(short, long)]
  pub feat: bool,

  #[arg(short = 'e', long)]
  pub interactive: bool,
}

pub fn cli(args: Cli) -> std::string::String {
  let type_message = match args.commit_type {
    CommitTypes { fix: true, .. } => "fix".to_owned(),
    CommitTypes { feat: true, .. } => "feat".to_owned(),
    _ => "".to_owned()
  };
  let scope = if args.scope.is_some() { format!("({})", args.scope.unwrap().to_owned()) } else { "".to_owned() };
  let is_breaking = if args.breaking_change { true } else { false };
  let breaking_mark = breaking_mark(is_breaking);
  let message = args.description.unwrap().to_owned();
  
  format!("{}{}{}:{}", type_message, scope, breaking_mark, message)
}

fn breaking_mark(is_breaking: bool) -> String {
  if is_breaking {
    "!".to_owned()
  } else {
    "".to_owned()
  }
}