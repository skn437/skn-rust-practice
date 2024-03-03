use serde::Deserialize;
//use std::collections::HashMap;

use config::{Config, ConfigError, File};

fn get_config() -> Config {
  let config: Result<Config, ConfigError> = Config::builder()
    .add_source(File::with_name("./rustenv"))
    .build();

  match config {
    | Ok(value) => value,
    | Err(e) => panic!("{}: Couldn't read config file! \n", e),
  }
}

// fn get_config_map() -> HashMap<String, String> {
//   let config: Config = get_config();

//   let config_deserialize: Result<HashMap<String, String>, ConfigError> = config.try_deserialize();

//   match config_deserialize {
//     | Ok(value) => value,
//     | Err(e) => {
//       print!("{}: Couldn't deserialize data from config file \n", e);
//       HashMap::new()
//     },
//   }
// }

// pub fn get_config_value(key: &str) -> String {
//   let config_map = get_config_map();

//   let get_config_value = config_map.get(key);

//   match get_config_value {
//     | Some(value) => value.to_string(),
//     | None => String::new(),
//   }
// }

#[derive(Debug, Deserialize)]
pub struct Author {
  pub name: String,
  pub age: String,
}

impl Author {
  fn new() -> Self {
    Self {
      name: String::from(""),
      age: String::from(""),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct ExternalConfig {
  pub author: Author,
}

impl ExternalConfig {
  pub fn new() -> Self {
    let config: Config = get_config();

    let config_deserialize: Result<Self, ConfigError> = config.try_deserialize();

    match config_deserialize {
      | Ok(value) => value,
      | Err(e) => {
        print!("{}: Couldn't deserialize data from config file \n", e);
        Self {
          author: Author::new(),
        }
      },
    }
  }
}
