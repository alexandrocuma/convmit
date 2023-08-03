use console::Style;
use clap::{Args, Parser};
use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[command(flatten)]
  commit_type: CommitTypes,

  #[arg(short, long)]
  scope: Option<String>,

  #[arg(short, long, required = true)]
  description: Option<String>,

  #[arg(short = 'o', long)]
  body: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct CommitTypes {
  #[arg(short = 'x', long)]
  fix: bool,

  #[arg(short, long)]
  feat: bool,
  
  #[arg(short, long)]
  breaking_change: bool,

  #[arg(short = 't', long)]
  interactive: bool,
}

fn main() {
  let args = Cli::parse();

  let cyan = Style::new().cyan();
  let green = Style::new().green();

  let mut conventional_commit = format!("{:?}{:?}: {:?} ", 
    match args.commit_type {
      CommitTypes { fix: true, .. } => "fix",
      CommitTypes { feat: true, .. } => "feat",
      CommitTypes { breaking_change: true, .. } => "BREAKING_CHANGE",
      _ => ""
    },
    if args.scope.is_some() { format!("({:?})", args.scope.unwrap()) } else { "".to_owned() },
    args.description.unwrap()
  );

  conventional_commit = conventional_commit.replace("\"", "").replace("\\", "");

  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  ctx.set_contents(conventional_commit.to_string()).unwrap();

  println!("\n");
  println!("{}", cyan.apply_to("The commit message has been copied to your clipboard!"));
  println!("Your message is: {}", green.apply_to(conventional_commit));
  println!("\n");
}