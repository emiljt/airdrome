pub struct Description {
    description: String,
}

impl Description {
    pub fn new(new_description: &str) -> Result<Description, &'static str> {
        Ok(Description {
            description: new_description.to_string(),
        })
    }
}
