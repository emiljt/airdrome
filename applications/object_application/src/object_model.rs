use super::name_model::Name;

pub struct Object {
    name: Name,
}

impl Object {
    pub fn new(name: Name) -> Object {
        Object {
            name,
        }
    }
}
