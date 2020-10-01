pub struct Url {
    address: String,
}

impl Url {
    pub fn new(new_address: &str) -> Result<Url, &'static str> {
        Ok(Url {
            address: new_address.to_string(),
        })
    }
}
