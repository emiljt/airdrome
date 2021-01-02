use super::description_factory;
use super::id_factory;
use super::languages_factory;
use super::name_factory;
use super::object_model::Object;
use super::targets_factory;
use super::versions_factory;
use super::versions_model::{Version, Versions};

pub fn create_object(
    name: &str,
    description: &str,
    languages: Vec<&str>,
    targets: Vec<&str>,
    versions: Vec<Version>,
) -> Result<Object, &'static str> {
    let new_id = id_factory::create_id(None)?;
    let new_name = name_factory::create_name(name)?;
    let new_description = description_factory::create_description(description)?;
    let new_languages = languages_factory::create_languages(languages)?;
    let new_targets = targets_factory::create_targets(targets)?;
    let new_versions = versions_factory::create_versions(versions)?;

    Ok(Object::new(
        new_id,
        new_name,
        new_description,
        new_languages,
        new_targets,
        new_versions,
    ))
}

pub fn restore_object(
    id: &str,
    name: &str,
    description: &str,
    languages: Vec<&str>,
    targets: Vec<&str>,
    versions: Versions,
) -> Result<Object, &'static str> {
    let new_id = id_factory::create_id(Some(id))?;
    let new_name = name_factory::create_name(name)?;
    let new_description = description_factory::create_description(description)?;
    let new_languages = languages_factory::create_languages(languages)?;
    let new_targets = targets_factory::create_targets(targets)?;

    Ok(Object::new(
        new_id,
        new_name,
        new_description,
        new_languages,
        new_targets,
        versions,
    ))
}
