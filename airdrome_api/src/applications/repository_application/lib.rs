mod id_factory;
mod id_model;
mod path_factory;
mod path_model;
mod repository_factory;
mod repository_model;
mod repository_repository;
mod url_factory;
mod url_model;
mod vcs_factory;
mod vcs_model;

use repository_model::Repository;

async fn add_new_repository(
    db_connection: sqlx::pool::PoolConnection<sqlx::MySql>,
    path: &str,
    url: &str,
) -> Result<RepositoryData, &'static str> {
    match git2::Repository::clone(url, path) {
        Ok(_) => (),
        Err(_) => return Err("Unable to create new repository"),
    }

    let new_repository = repository_factory::create_repository("git", url, path)?;

    return match repository_repository::save_repository(db_connection, &new_repository).await {
        Ok(_) => Ok(RepositoryData::from_repository(new_repository)),
        Err(_) => Err("Unable to save repository"),
    };
}

struct RepositoryData {
    path: String,
    vcs: String,
    url: String,
}

impl RepositoryData {
    fn from_repository(repo: Repository) -> RepositoryData {
        RepositoryData {
            path: "".to_string(),
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
