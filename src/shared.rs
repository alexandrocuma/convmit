
use std::{fs::File, io::{BufReader, BufRead}};
use dialoguer::{Confirm, MultiSelect};

fn read_coauthors() -> Vec<String> {
  let file = File::open("config/co-authors.txt").unwrap();
  let reader = BufReader::new(file);
  reader.lines().map(|line| line.unwrap()).filter(|line| !line.is_empty()).collect::<Vec<String>>()
}

pub fn set_co_authors() -> String {
  if Confirm::new().with_prompt("Do you have co-authors in this commit?").interact().unwrap() {
    let co_authors = read_coauthors();
    let selection : Vec<usize> = MultiSelect::new()
      .with_prompt("Select the co-authors")
      .items(&co_authors)
      .interact()
      .unwrap();

    "\n".to_owned() + &selection.iter().map(|index| "Co-authored-by: ".to_owned() + &co_authors[*index].to_owned()).collect::<Vec<String>>().join("\n")
  } else {
    "".to_owned()
  }
}