mod description_factory;
mod description_model;
mod id_factory;
mod id_model;
mod languages_factory;
mod languages_model;
mod name_factory;
mod name_model;
mod object_factory;
mod object_model;
mod object_repository;
mod targets_factory;
mod targets_model;

use std::convert::From;

pub async fn add_new_object(
    db_connection: sqlx::pool::PoolConnection<sqlx::MySql>,
    name: &str,
    description: &str,
    languages: Vec<&str>,
    targets: Vec<&str>,
) -> Result<Object, &'static str> {
    let new_object = object_factory::create_object(name, description, languages, targets)?;
    object_repository::save_object(db_connection, &new_object).await?;

    Ok(Object::from_object_model(new_object))
}

pub async fn find_object(
    db_connection: sqlx::pool::PoolConnection<sqlx::MySql>,
    id: &str,
) -> Result<Object, &'static str> {
    match object_repository::read_object(db_connection, id).await {
        Ok(object) => Ok(Object::from_object_model(object)),
        Err(_) => Err("No object found"),
    }
}

pub async fn search_objects(
    db_connection: sqlx::pool::PoolConnection<sqlx::MySql>,
    name: Option<&str>,
    targets: Option<Vec<&str>>,
    languages: Option<Vec<&str>>,
    keywords: Option<&str>,
    categories: Option<Vec<&str>>,
    created: Option<&str>,
    updated: Option<&str>,
) -> Vec<Object> {
    let mut results = Vec::new();

    match object_repository::read_objects(db_connection, name, targets, languages, keywords).await {
        Ok(objects) => {
            for result in objects {
                results.push(Object::from_object_model(result));
            }

            results
        }
        Err(e) => panic!(e),
    }
}

pub struct Object {
    pub id: String,
    pub name: String,
    pub targets: Vec<Target>,
    pub languages: Vec<Language>,
}

impl Object {
    fn from_object_model(object_model: object_model::Object) -> Object {
        let mut targets: Vec<Target> = Vec::new();
        let mut languages: Vec<Language> = Vec::new();

        for target in object_model.targets.value {
            targets.push(Target::from(target));
        }

        for language in object_model.languages.value {
            languages.push(Language::from(language));
        }

        Object {
            id: object_model.id.value,
            name: object_model.name.value,
            targets: targets,
            languages: languages,
        }
    }
}

#[derive(Debug)]
pub enum Target {
    BS1,
    BS2,
    BS2E,
    BS2SX,
    BS2P24,
    BS2P40,
    BS2PE,
    BS2PX,
    SX,
    P1,
    P2,
}

impl From<targets_model::Target> for Target {
    fn from(item: targets_model::Target) -> Target {
        match item {
            targets_model::Target::BS1 => Target::BS1,
            targets_model::Target::BS2 => Target::BS2,
            targets_model::Target::BS2E => Target::BS2E,
            targets_model::Target::BS2SX => Target::BS2SX,
            targets_model::Target::BS2P24 => Target::BS2P24,
            targets_model::Target::BS2P40 => Target::BS2P40,
            targets_model::Target::BS2PE => Target::BS2PE,
            targets_model::Target::BS2PX => Target::BS2PX,
            targets_model::Target::SX => Target::SX,
            targets_model::Target::P1 => Target::P1,
            targets_model::Target::P2 => Target::P2,
        }
    }
}

#[derive(Debug)]
pub enum Language {
    Spin,
    Spin2,
    Pasm,
    Pasm2,
    C,
    Basic,
    Forth,
    Python,
}

impl From<languages_model::Language> for Language {
    fn from(item: languages_model::Language) -> Language {
        match item {
            languages_model::Language::Spin => Language::Spin,
            languages_model::Language::Spin2 => Language::Spin2,
            languages_model::Language::Pasm => Language::Pasm,
            languages_model::Language::Pasm2 => Language::Pasm2,
            languages_model::Language::C => Language::C,
            languages_model::Language::Basic => Language::Basic,
            languages_model::Language::Forth => Language::Forth,
            languages_model::Language::Python => Language::Python,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
