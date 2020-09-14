use super::description_model::Description;

pub fn create_description(description: &str) -> Result<Description, &'static str> {
    if description.chars().count() > 2500 {
        Err("Description must be less than 2500 chracters")
    } else {
        Description::new(description)
    }
}
