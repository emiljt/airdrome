#[derive(Clone)]
pub struct Description {
    pub value: String,
}

impl Description {
    pub fn new(new_description: &str) -> Result<Description, &'static str> {
        Ok(Description {
            value: new_description.to_string(),
        })
    }
}
