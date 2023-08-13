use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub coauthors: Vec<String>,
    pub scopes: Vec<String>,
}

pub fn create_configuration(folder_path: &str) {
    let mut home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Can't determine the root directory."),
    };
    home_dir.push(folder_path);

    let folder_path = Path::new(&home_dir).to_str().expect(
        format!(
            "Can't find your config file. Make sure ~/{} exists",
            folder_path
        )
        .as_str(),
    );

    if !std::path::Path::new(folder_path).exists() {
        match std::fs::create_dir(folder_path) {
            Ok(..) => {
                create_file(home_dir);
            }
            Err(e) => eprintln!("Error while creating folder: {}", e),
        };
    } else {
        create_file(home_dir);
    }
}

pub fn get_configuration(file_path: &str) -> Result<Settings, config::ConfigError> {
    let mut home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Can't determine the root directory."),
    };

    home_dir.push(file_path);

    let path = Path::new(&home_dir)
        .to_str()
        .expect(format!("Unable to create the folder").as_str());

    let settings = config::Config::builder()
        .add_source(config::File::new(path, config::FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<Settings>()
}

fn create_file(mut home_dir: PathBuf) {
    home_dir.push("config.yml");

    let file_path = Path::new(&home_dir)
        .to_str()
        .expect(format!("Unable to create the config file").as_str());

    match File::create(&file_path) {
        Ok(mut file) => {
            let content = "coauthors:\n  - John Doe <johndoe@test.com>\n  - Jane Doe<janedoe@test.com>\nscopes:\n  - test_1";
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Error while writing to file: {}", e);
            }
        }
        Err(e) => eprintln!("Error while creating file: {}", e),
    };
}

#[cfg(test)]
mod tests {
    use std::fs::{remove_file, File};
    use std::io::Write;

    #[test]
    fn it_gets_configuration() {
        // Test setup
        let test_file_path = ".convmit/test.yml";
        let mut home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => panic!("Can't determine the root directory."),
        };

        home_dir.push(test_file_path);

        match File::create(&home_dir) {
            Ok(mut file) => {
                let content = "coauthors:\n  - John Doe\n  - Jane Doe\nscopes:\n  - test_1";
                if let Err(e) = file.write_all(content.as_bytes()) {
                    eprintln!("Error while writing to file: {}", e);
                }
            }
            Err(e) => eprintln!("Error while creating file: {}", e),
        }

        // Test assert
        let settings = super::get_configuration(test_file_path).unwrap();
        assert_eq!(settings.coauthors.len(), 2);
        assert_eq!(settings.scopes.len(), 1);
        remove_file(&home_dir).unwrap();
    }

    #[test]
    fn it_panics_when_configuration_file_does_not_exist() {
        let settings = super::get_configuration(".convmit/config.yaaaml");
        assert!(settings.is_err());
    }
}
