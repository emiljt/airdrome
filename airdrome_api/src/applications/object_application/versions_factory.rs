use super::hash_factory;
use super::id_factory;
use super::timestamp_factory;
use super::version_number_factory;
use super::versions_model::{Version, Versions};

pub fn create_versions(versions: Vec<Version>) -> Result<Versions, &'static str> {
    Ok(Versions::new(versions))
}

pub fn create_version(
    number: Option<&str>,
    commit: &str,
    zip_hash: &str,
) -> Result<Version, &'static str> {
    let new_id = id_factory::create_id(None)?;
    let new_version_number = version_number_factory::create_version_number(number.unwrap_or(""))?;
    let new_commit = hash_factory::create_hash(commit)?;
    let new_zip_hash = hash_factory::create_hash(zip_hash)?;
    let new_created_timestamp = timestamp_factory::create_timestamp(None)?;

    Ok(Version::new(
        new_id,
        new_version_number,
        new_commit,
        new_zip_hash,
        new_created_timestamp,
    ))
}

pub fn restore_version(
    id: &str,
    number: &str,
    commit: &str,
    zip_hash: &str,
    create_timestamp: &str,
) -> Result<Version, &'static str> {
    let id = id_factory::create_id(Some(id))?;
    let number = version_number_factory::create_version_number(number)?;
    let commit = hash_factory::create_hash(commit)?;
    let zip_hash = hash_factory::create_hash(zip_hash)?;
    let created_timestamp = timestamp_factory::create_timestamp(Some(create_timestamp))?;

    Ok(Version::new(
        id,
        number,
        commit,
        zip_hash,
        created_timestamp,
    ))
}
