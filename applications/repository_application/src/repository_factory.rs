fn create_repository(
    type: &str,
    url: &str,
) -> Result<Repository, &'static str> {
    if let Some("git") == type {
        Type::Git
    }
    let new_url = url_factory::create_url(url)?;

    Ok(Repository::new(new_type, new_url))
}