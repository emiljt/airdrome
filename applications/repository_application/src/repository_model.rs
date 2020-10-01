use super::url_model::Url;
use super::vcs_model::Vcs;

pub struct Repository {
    vcs: Vcs,
    url: Url,
}

impl Repository {
    pub fn new(vcs: Vcs, url: Url) -> Repository {
        Repository { vcs, url }
    }
}
