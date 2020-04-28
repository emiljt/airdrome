use super::vcs_model::Vcs;
use super::url_model::Url;

struct Repository {
    vcs: Vcs,
    url: Url,
}

impl Repository {
    fn new(vcs, url) -> Repository {
        Repository {
            vcs,
            url,
        }
    }
}
