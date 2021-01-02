use std::io::prelude::*;
use zip;

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

    loop {
        let current_path = match queue.len() {
            0 => target.to_path_buf(),
            _ => queue.pop().expect("Unable to get next path"),
        };

        if current_path.to_str() == target.join(name).to_str() {
            continue;
        }

        if std::fs::metadata(&current_path)
            .expect("Unable to open path")
            .is_dir()
        {
            // let relative_path = current_path
            //     .to_str()
            //     .expect("Unable to convert current path to string")
            //     .replace(
            //         target
            //             .to_str()
            //             .expect("Unable to convert target path to string"),
            //         "",
            //     );

            // zip.add_directory(relative_path, zip_options)?;

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
                path.file_name()
                    .expect("Unable to get file name")
                    .to_str()
                    .expect("Unable to convert file name to string"),
                zip_options,
            )?;
            zip.write_all(&buffer)?;
        }

        if queue.len() == 0 {
            break;
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
