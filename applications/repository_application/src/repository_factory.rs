use super::repository_model::Repository;
use super::url_factory;
use super::vcs_model::Vcs;

pub fn create_repository(vcs: &str, url: &str) -> Result<Repository, &'static str> {
    let new_vcs: Vcs;
    let new_url = url_factory::create_url(url)?;

    if let "git" = vcs {
        new_vcs = Vcs::Git;
    } else {
        return Err("No VCS provided");
    }

    Ok(Repository::new(new_vcs, new_url))
}
