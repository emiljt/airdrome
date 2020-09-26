use super::id_model::Id;
use super::name_model::Name;
use super::description_model::Description;
use super::languages_model::Languages;
use super::targets_model::Targets;

#[derive(Clone)]
pub struct Object {
    pub id: Id,
    pub name: Name,
    pub description: Description,
    pub languages: Languages,
    pub targets: Targets,
}

impl Object {
    pub fn new(id: Id, name: Name, description: Description, languages: Languages, targets: Targets)
        -> Object {
        Object {
            id,
            name,
            description,
            languages,
            targets,
        }
    }
}
