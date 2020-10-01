use super::id_model::Id;

pub fn create_id(id: Option<&str>) -> Result<Id, &'static str> {
    match id {
        Some(i) => match uuid::Uuid::parse_str(i) {
            Ok(r) => Id::new(&r.to_string()),
            Err(_) => Err("Invalid object guid"),
        },
        None => Id::new(&uuid::Uuid::new_v4().to_string()),
    }
}
