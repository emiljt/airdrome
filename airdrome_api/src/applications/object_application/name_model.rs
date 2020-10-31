#[derive(Clone)]
pub struct Name {
    pub value: String,
}

impl Name {
    pub fn new(new_name: &str) -> Result<Name, &'static str> {
        Ok(Name {
            value: new_name.to_string(),
        })
    }
}
