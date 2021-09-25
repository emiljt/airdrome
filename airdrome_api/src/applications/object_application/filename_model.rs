pub struct Filename {
    pub value: String,
}

impl Filename {
    pub fn new(new_filename: &str) -> Result<Filename, &'static str> {
        Ok(Filename {
            value: new_filename.to_string(),
        })
    }
}
