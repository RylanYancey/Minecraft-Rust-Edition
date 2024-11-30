use std::path::PathBuf;
use std::{fs, io};

use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Resource, Default, Deserialize)]
pub struct Locale {
    pub map: HashMap<String, String>,
    pub sources: Vec<PathBuf>,
    pub lang: String,
}

impl Locale {
    pub fn load(&mut self, lang: &str) -> Result<(), Error> {
        self.lang = lang.to_string();

        for source in &self.sources {
            let langfile = source.join(lang).with_extension("json");
            if let Ok(src) = fs::read_to_string(&langfile) {
                match serde_json::from_str::<HashMap<String, String>>(&*src) {
                    Ok(map) => self.map.extend(map.into_iter()),
                    Err(err) => {
                        return Err(Error::JsonError {
                            error: err,
                            langfile,
                        })
                    }
                }
            } else {
                log::warn!("Attempted to load langfile at {langfile:?}, but it did not exist.");
            }
        }

        Ok(())
    }

    /// Add a DIRECTORY of language files.
    pub fn add_source(&mut self, path: PathBuf) {
        self.sources.push(path);
    }

    pub fn get(&self, key: &str) -> &str {
        self.map.get(key).unwrap_or_else(|| {
            panic!("Attempted to get localization with '{key}', but it was not found.");
        })
    }
}

#[derive(Debug)]
pub enum Error {
    JsonError {
        error: serde_json::Error,
        langfile: PathBuf,
    },

    IoError {
        error: io::Error,
        langfile: PathBuf,
    },
}

/// TODO - save lang
pub fn load_locale(mut locale: ResMut<Locale>) {
    locale.add_source(PathBuf::from("./assets/lang/"));
    locale.load("en-us").expect("Failed to load Localization!");
}
