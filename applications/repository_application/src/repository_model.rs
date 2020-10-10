use super::id_model::Id;
use super::path_model::Path;
use super::url_model::Url;
use super::vcs_model::Vcs;

pub struct Repository {
    pub id: Id,
    pub vcs: Vcs,
    pub path: Path,
    pub url: Url,
}

impl Repository {
    pub fn new(id: Id, vcs: Vcs, path: Path, url: Url) -> Repository {
        Repository { id, path, vcs, url }
    }
}
