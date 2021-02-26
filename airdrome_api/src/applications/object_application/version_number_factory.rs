use super::version_number_model::VersionNumber;

pub fn create_version_number(version_number: &str) -> Result<VersionNumber, &'static str> {
    if version_number.chars().count() > 32 {
        Err("Name must be less than 32 chracters")
    } else {
        VersionNumber::new(version_number)
    }
}
