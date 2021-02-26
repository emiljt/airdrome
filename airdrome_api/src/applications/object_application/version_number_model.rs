#[derive(Clone)]
pub struct VersionNumber {
    pub value: String,
}

impl VersionNumber {
    pub fn new(new_version_number: &str) -> Result<VersionNumber, &'static str> {
        Ok(VersionNumber {
            value: new_version_number.to_string(),
        })
    }
}
