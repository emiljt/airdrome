pub struct Name {
    name: String,
}

impl Name {
    pub fn new(new_name: &str) -> Result<Name, &'static str> {
        Ok(Name {
            name: new_name.to_string(),
        })
    }
}
