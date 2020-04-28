struct Url {
    address: String
}

impl Url {
    fn new(new_address: &str) -> Result(URL, &'static str) {
        Ok(Url {
            address: new_address.to_string()
        })
    }
}
