use super::name_model::Name;
use super::description_model::Description;
use super::languages_model::Languages;
use super::targets_model::Targets;

pub struct Object {
    name: Name,
    description: Description,
    languages: Languages,
    targets: Targets,
}

impl Object {
    pub fn new(name: Name, description: Description, languages: Languages, targets: Targets)
        -> Object {
        Object {
            name,
            description,
            languages,
            targets,
        }
    }
}
