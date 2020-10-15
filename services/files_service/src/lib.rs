use std::io::prelude::*;

pub fn create_zip_file(
    target: &std::path::Path,
    destination: &std::path::Path,
    name: &str,
) -> Result<std::path::PathBuf, std::io::Error> {
    let mut queue: Vec<std::path::PathBuf> = Vec::new();
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination.join(name))?;
    let mut zip = zip::ZipWriter::new(file);
    let zip_options = zip::write::FileOptions::default();

    queue.push(target.to_path_buf());

    while queue.len() > 0 {
        let current_path = queue.pop().expect("Unable to get next path");

        if current_path.to_str() == target.join(name).to_str() {
            continue;
        }

        if std::fs::metadata(&current_path)
            .expect("Enable to open path")
            .is_dir()
        {
            zip.add_directory(
                current_path
                    .to_str()
                    .expect("Unable to convert path to string"),
                zip_options,
            )?;

            for item in std::fs::read_dir(&current_path)? {
                let item = item?;
                queue.push(item.path());
            }
        } else {
            let path = std::path::Path::new(&current_path);
            let mut current_file = std::fs::File::open(path)?;
            let mut buffer = Vec::new();

            current_file.read_to_end(&mut buffer)?;
            zip.start_file(
                current_path
                    .to_str()
                    .expect("Unable to convert path to string"),
                zip_options,
            )?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(destination.join(name))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
