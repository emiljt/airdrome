use super::description_model::Description;
use super::id_model::Id;
use super::languages_model::Languages;
use super::name_model::Name;
use super::targets_model::Targets;
use super::versions_model::Versions;

#[derive(Clone)]
pub struct Object {
    pub id: Id,
    pub name: Name,
    pub description: Description,
    pub languages: Languages,
    pub targets: Targets,
    pub versions: Versions,
}

impl Object {
    pub fn new(
        id: Id,
        name: Name,
        description: Description,
        languages: Languages,
        targets: Targets,
        versions: Versions,
    ) -> Object {
        Object {
            id,
            name,
            description,
            languages,
            targets,
            versions,
        }
    }
}
