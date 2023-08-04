use dialoguer::{console::Term, theme::ColorfulTheme, Select, Input, Confirm};

use crate::shared::set_co_authors;

enum CommitTypes {
  Fix, 
  Feat,
  Docs, 
  Style,
  Refactor,
  Perf,
  Test,
  Chore
}

impl CommitTypes {
  fn select_message(&self) -> &'static str {
    match self {
      Self::Feat      => "feat:     A code change that adds a new feature",
      Self::Fix       => "fix:      A code change that fixes a bug",
      Self::Docs      => "docs:     A documentation change",
      Self::Style     => "style:    A code change that adds/fixes any style",
      Self::Refactor  => "refactor: A code change that not fixes or adds anything new",
      Self::Perf      => "perf:     A code change that improves performance",
      Self::Test      => "test:     A code change that adds/fixes tests",
      Self::Chore     => "chore:    Changes on build process or auxiliary tools and libraries (e.g. doc generation)"
    }
  } 
  fn value(&self) -> &'static str {
    match self {
      Self::Feat      => "feat",
      Self::Fix       => "fix",
      Self::Docs      => "docs",
      Self::Style     => "style",
      Self::Refactor  => "refactor",
      Self::Perf      => "perf",
      Self::Test      => "test",
      Self::Chore     => "chore"
    }
  } 
}

pub fn cli() -> std::string::String {
  let type_message = commit_type();
  let scope = scope_message();
  let is_breaking = is_breaking_change();
  let breaking_mark = breaking_mark(is_breaking);
  let message = description_message();
  let co_authors = set_co_authors();
  
  format!("{}{}{}:{}{}", type_message, scope, breaking_mark, message, co_authors)
}

fn commit_type() -> String {
  let select_messages = vec![
    CommitTypes::Feat.select_message(),
    CommitTypes::Fix.select_message(),
    CommitTypes::Docs.select_message(),
    CommitTypes::Style.select_message(),
    CommitTypes::Refactor.select_message(),
    CommitTypes::Perf.select_message(),
    CommitTypes::Test.select_message(),
    CommitTypes::Chore.select_message(),
  ];

  let select_values = vec![
    CommitTypes::Feat.value(),
    CommitTypes::Fix.value(),
    CommitTypes::Docs.value(),
    CommitTypes::Style.value(),
    CommitTypes::Refactor.value(),
    CommitTypes::Perf.value(),
    CommitTypes::Test.value(),
    CommitTypes::Chore.value(),
  ];

  let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Select the type of change that you´re commiting: (Use arrow keys)")
    .items(&select_messages)
    .default(0)
    .interact_on_opt(&Term::stderr())
    .unwrap();

  match selection {
    Some(index) => select_values[index],
    None => panic!("Error not type selected")
  }.to_owned()
}

fn is_breaking_change() -> bool {
  if Confirm::new().with_prompt("Is it a breaking change?").interact().unwrap() {
    true
  } else {
    false
  }
}

fn breaking_mark(is_breaking: bool) -> String {
  if is_breaking {
    "!".to_owned()
  } else {
    "".to_owned()
  }
}

fn scope_message() -> String {
  let input : String = Input::<String>::new()
    .with_prompt("Scope message (optional)")
    .allow_empty(true)
    .interact_text()
    .unwrap();

  if !input.is_empty() {
    format!("({})", input)
  } else {
    "".to_owned()
  }
}

fn description_message() -> String {
  return Input::<String>::new()
    .with_prompt("Your description message")
    .interact_text()
    .unwrap();
}
