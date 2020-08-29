use super::vcs_model::Vcs;
use super::url_model::Url;

pub struct Repository {
    vcs: Vcs,
    url: Url,
}

impl Repository {
    pub fn new(vcs: Vcs, url: Url) -> Repository {
        Repository {
            vcs,
            url,
        }
    }
}
