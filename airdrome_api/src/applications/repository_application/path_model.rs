pub struct Path {
    pub value: String,
}

impl Path {
    pub fn new(new_path: &str) -> Result<Path, &'static str> {
        Ok(Path {
            value: new_path.to_string(),
        })
    }
}
