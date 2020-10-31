use super::name_model::Name;

pub fn create_name(name: &str) -> Result<Name, &'static str> {
    if name.is_empty() {
        Err("Name can't be empty")
    } else if name.chars().count() > 100 {
        Err("Name must be less than 100 chracters")
    } else {
        Name::new(name)
    }
}
