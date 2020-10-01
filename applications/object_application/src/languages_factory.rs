use super::languages_model::{Language, Languages};
use std::convert::TryFrom;

pub fn create_languages(languages: Vec<&str>) -> Result<Languages, &'static str> {
    let mut new_languages: Vec<Language> = Vec::new();

    for language in languages {
        match Language::try_from(language) {
            Ok(i) => new_languages.push(i),
            Err(_) => return Err("Invalid language"),
        }
    }

    Ok(Languages::new(new_languages))
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "spin" => Ok(Language::Spin),
            "spin2" => Ok(Language::Spin2),
            "pasm" => Ok(Language::Pasm),
            "pasm2" => Ok(Language::Pasm2),
            "c" => Ok(Language::C),
            "basic" => Ok(Language::Basic),
            "forth" => Ok(Language::Forth),
            "python" => Ok(Language::Python),
            _ => Err("Not a valid language"),
        }
    }
}
