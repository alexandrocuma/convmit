use crate::configuration;
use dialoguer::{Confirm, MultiSelect};

fn read_coauthors() -> Vec<String> {
    let settings = match configuration::get_configuration(".convmit/config.yml") {
        Ok(settings) => settings,
        Err(err) => panic!("{}", err),
    };

    settings.coauthors
}

pub fn set_co_authors() -> String {
    if Confirm::new()
        .with_prompt("Do you have co-authors in this commit?")
        .interact()
        .unwrap()
    {
        let co_authors = read_coauthors();
        let selection: Vec<usize> = MultiSelect::new()
            .with_prompt("Select the co-authors")
            .items(&co_authors)
            .interact()
            .unwrap();

        "\n\n".to_owned()
            + &selection
                .iter()
                .map(|index| "Co-authored-by: ".to_owned() + &co_authors[*index].to_owned())
                .collect::<Vec<String>>()
                .join("\n")
    } else {
        "".to_owned()
    }
}
