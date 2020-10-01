mod repository_factory;
mod repository_model;
mod url_factory;
mod url_model;
mod vcs_factory;
mod vcs_model;

use repository_model::Repository;

fn add_new_repository(url: &str) -> Result<RepositoryData, &'static str> {
    let new_repository = repository_factory::create_repository("git", url)?;

    Ok(RepositoryData::from_repository(new_repository))
}

struct RepositoryData {
    vcs: String,
    url: String,
}

impl RepositoryData {
    fn from_repository(repo: Repository) -> RepositoryData {
        RepositoryData {
            vcs: "".to_string(),
            url: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
