use dialoguer::{console::Term, theme::ColorfulTheme, Select, Input, Confirm};

enum CommitTypes {
  Fix, 
  Feat,
}

impl CommitTypes {
  fn select_message(&self) -> &'static str {
    match self {
      Self::Feat => "feat: a new feature",
      Self::Fix =>  "fix: a bug fix",
    }
  } 
  fn value(&self) -> &'static str {
    match self {
      Self::Feat => "feat",
      Self::Fix =>  "fix",
    }
  } 
}

pub fn cli() -> std::string::String {
  let type_message = commit_type();
  let scope = scope_message();
  let is_breaking = is_breaking_change();
  let breaking_mark = breaking_mark(is_breaking);
  let message = description_message();
  
  format!("{}{}{}:{}", type_message, scope, breaking_mark, message)
}

fn commit_type() -> String {
  let select_messages = vec![
    CommitTypes::Fix.select_message(),
    CommitTypes::Feat.select_message(),
  ];

  let select_values = vec![
    CommitTypes::Fix.value(),
    CommitTypes::Feat.value(),
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

