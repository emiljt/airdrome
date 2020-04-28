fn create_repository(
    vcs: &str,
    url: &str,
) -> Result<Repository, &'static str> {
    if let Some("git") == vcs {
        Vcs::Git
    }
    let new_url = url_factory::create_url(url)?;

    Ok(Repository::new(new_vcs, new_url))
}