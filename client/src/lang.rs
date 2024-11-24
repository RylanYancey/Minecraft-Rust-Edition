
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Resource, Default, Deserialize)]
pub struct Locale(HashMap<String, String>);

impl Locale {
    pub fn load(&mut self, lang: &str) {
        let str = std::fs::read_to_string(format!("assets/lang/{lang}.json")).expect(&format!("Language file '{lang}' doe snot exist!"));
         *self = serde_json::from_str(&*str).expect(&format!("Error in Lang File: {lang}"));
    }

    pub fn get(&self, key: &str) -> &str {
        self.0.get(key).unwrap_or_else(|| {
            panic!("Attempted to get localization with '{key}', but it was not found.");
        })
    }
}

/// TODO - save lang
pub fn load_locale(
    mut locale: ResMut<Locale>,
) {
    locale.load("en-us");
}
