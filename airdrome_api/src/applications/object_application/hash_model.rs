#[derive(Clone)]
pub struct Hash {
    pub value: String,
}

impl Hash {
    pub fn new(new_hash: &str) -> Result<Hash, &'static str> {
        Ok(Hash {
            value: new_hash.to_string(),
        })
    }
}
