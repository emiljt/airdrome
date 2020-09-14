use super::object_model::Object;
use super::name_factory;
use super::description_factory;
use super::languages_factory;
use super::targets_factory;

pub fn create_object(name: &str, description: &str, languages: Vec<&str>, targets: Vec<&str>)
    -> Result<Object, &'static str> {
    let new_name = name_factory::create_name(name)?;
    let new_description = description_factory::create_description(description)?;
    let new_languages = languages_factory::create_languages(languages)?;
    let new_targets = targets_factory::create_targets(targets)?;

    Ok(Object::new(new_name, new_description, new_languages, new_targets))
}
