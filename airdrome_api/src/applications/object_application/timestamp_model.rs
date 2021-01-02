#[derive(Clone)]
pub struct Timestamp {
    pub value: String,
}

impl Timestamp {
    pub fn new(new_timestamp: &str) -> Result<Timestamp, &'static str> {
        Ok(Timestamp {
            value: new_timestamp.to_string(),
        })
    }
}
