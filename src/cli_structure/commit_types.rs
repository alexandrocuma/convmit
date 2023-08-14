use clap::Subcommand;

#[derive(Debug, Subcommand)]
// #[group(required = true, multiple = false)]
pub enum CommitTypes {
    Fix,
    Feat,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
    Chore,
}

impl CommitTypes {
  pub fn select_message(&self) -> &'static str {
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
  
  pub fn value(&self) -> &'static str {
      match self {
          Self::Feat => "feat",
          Self::Fix => "fix",
          Self::Docs => "docs",
          Self::Style => "style",
          Self::Refactor => "refactor",
          Self::Perf => "perf",
          Self::Test => "test",
          Self::Chore => "chore",
      }
  }
}