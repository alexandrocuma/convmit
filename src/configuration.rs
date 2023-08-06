use std::path::Path;

#[derive(serde::Deserialize)]
pub struct Settings {
  pub coauthors: Vec<String>,
  pub scopes: Vec<String>,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  let mut home_dir = match dirs::home_dir() {
    Some(path) => path,
    None => panic!("Can't determine the root directory.")
  };

  let file_path = ".convmit/config.yml";
  home_dir.push(file_path);

  let path = Path::new(&home_dir).to_str().expect(format!("Can't find your config file. Make sure ~/{} exists", file_path).as_str());

  let settings = config::Config::builder()
    .add_source(config::File::new(path, config::FileFormat::Yaml))
    .build()?;

  settings.try_deserialize::<Settings>()
}
