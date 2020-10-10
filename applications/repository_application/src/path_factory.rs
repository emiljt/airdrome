use super::path_model::Path;

pub fn create_path(path: &str) -> Result<Path, &'static str> {
    let new_path = std::fs::read_dir(path).expect("Unable to parse path");

    Path::new(path)
}
