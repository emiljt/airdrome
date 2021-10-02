use super::filename_model::Filename;
use regex::Regex;

pub fn create_filename(
    object_name: &str,
    version_number: Option<&str>,
) -> Result<Filename, &'static str> {
    // Remove some characters rather than replace them
    let remove_regex =
        Regex::new(r"[\[\]]").expect("Unable to initialize regular expression for object filename");
    // Replace all but safe character
    let replace_regex = Regex::new(r"[^0-9a-zA-Z_\-]")
        .expect("Unable to initialize regular expression for object filename");

    let file_version_number = match version_number {
        Some(v) => {
            if v.is_empty() {
                "latest"
            } else {
                v
            }
        }
        None => "latest",
    };

    let mut file_object_name = remove_regex.replace_all(object_name, "");
    file_object_name = replace_regex.replace_all(object_name, "_");

    let filename = format!("{}-{}", file_object_name, file_version_number);

    if filename.chars().count() > 144 {
        Err("Filename must be less than 144 chracters")
    } else {
        Filename::new(&filename)
    }
}
