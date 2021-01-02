#[derive(Clone)]
pub struct Id {
    pub value: String,
}

impl Id {
    pub fn new(new_id: &str) -> Result<Id, &'static str> {
        Ok(Id {
            value: new_id.to_string(),
        })
    }

    pub fn uuid(&self) -> String {
        self.value.replace("-", "")
    }
}
