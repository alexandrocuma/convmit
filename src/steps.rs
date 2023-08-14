
use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Input, Select};

use crate::{coauthors::set_coauthors, cli_structure::commit_types::CommitTypes, command_line::gen_args::GenArgs};

#[derive(Debug, Default)]
pub struct Steps {
  type_message: String,
  scope: String,
  is_breaking: bool,
  breaking_mark: String,
  message: String,
  coauthors: String
}

impl Steps {
    pub fn new() -> Self {
      Steps { ..Default::default() }
    }

    pub fn generate_command(&mut self, args: &GenArgs) -> std::string::String {
        self.type_message = match &args.commit_type {
          commit_type => commit_type.value(),
        }.to_owned();

        if args.scope.is_some() {
          self.scope_message(args.scope.as_ref().unwrap().to_owned());
        }

        self.is_breaking = args.breaking_change;
        self.breaking_mark();

        self.description_message(args.description.as_ref().unwrap().to_owned());
        self.coauthors = set_coauthors();

        self.set_message()
    }

    pub fn generate_interactive(&mut self) -> std::string::String {
        self.commit_type();
        self.scope_step();
        self.is_breaking_change();
        self.breaking_mark();
        self.description_step();
        self.coauthors = set_coauthors();

        self.set_message()
    }

    fn set_message(&self) -> String {
      format!(
        "{}{}{}: {}{}",
        self.type_message, self.scope, self.breaking_mark, self.message, self.coauthors
      )
    }

    fn commit_type(&mut self) {
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
    
        self.type_message = match selection {
            Some(index) => select_values[index],
            None => panic!("Error not type selected"),
        }
        .to_owned()
    }

    fn is_breaking_change(&mut self) {
      self.is_breaking = if Confirm::new()
          .with_prompt("Is it a breaking change?")
          .interact()
          .unwrap()
      {
          true
      } else {
          false
      }
    }

    fn breaking_mark(&mut self) {
      self.breaking_mark = if self.is_breaking {
          "!".to_owned()
      } else {
          "".to_owned()
      }
    }

    fn scope_step(&mut self) {
      let input: String = Input::<String>::new()
          .with_prompt("Scope message (optional)")
          .allow_empty(true)
          .interact_text()
          .unwrap();

     self.scope_message(input)
    }

    fn scope_message(&mut self, mesage: String) {
      self.scope = if !mesage.is_empty() {
          format!("({})", mesage)
      } else {
          "".to_owned()
      }
    }

    fn description_step(&mut self) {
      let input = Input::<String>::new()
          .with_prompt("Your description message")
          .interact_text()
          .unwrap();

      self.description_message(input);
    }

    fn description_message(&mut self, message: String) {
      self.message = message;
    }
}