use crate::{
    cli_struct::{CommitTypes, GenArgs},
    shared::set_co_authors,
};

pub fn cli(args: &GenArgs) -> std::string::String {
    let type_message = match args.commit_type {
        CommitTypes { fix: true, .. } => "fix".to_owned(),
        CommitTypes { feat: true, .. } => "feat".to_owned(),
        CommitTypes { docs: true, .. } => "docs".to_owned(),
        CommitTypes { style: true, .. } => "style".to_owned(),
        CommitTypes { refactor: true, .. } => "refactor".to_owned(),
        CommitTypes { perf: true, .. } => "perf".to_owned(),
        CommitTypes { test: true, .. } => "test".to_owned(),
        CommitTypes { chore: true, .. } => "chore".to_owned(),
        _ => "".to_owned(),
    };
    let scope = if args.scope.is_some() {
        format!("({})", args.scope.as_ref().unwrap().to_owned())
    } else {
        "".to_owned()
    };
    let is_breaking = if args.breaking_change { true } else { false };
    let breaking_mark = breaking_mark(is_breaking);
    let message = args.description.as_ref().unwrap().to_owned();
    let co_authors = set_co_authors();

    format!(
        "{}{}{}: {}{}",
        type_message, scope, breaking_mark, message, co_authors
    )
}

fn breaking_mark(is_breaking: bool) -> String {
    if is_breaking {
        "!".to_owned()
    } else {
        "".to_owned()
    }
}
