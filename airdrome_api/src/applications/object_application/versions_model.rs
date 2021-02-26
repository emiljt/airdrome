use super::hash_model::Hash;
use super::id_model::Id;
use super::timestamp_model::Timestamp;
use super::version_number_model::VersionNumber;

#[derive(Clone)]
pub struct Versions {
    pub all: Vec<Version>,
}

#[derive(Clone)]
pub struct Version {
    pub id: Id,
    pub number: VersionNumber,
    pub commit: Hash,
    pub zip_hash: Hash,
    pub created_timestamp: Timestamp,
}

impl Versions {
    pub fn new(versions: Vec<Version>) -> Versions {
        Versions { all: versions }
    }

    pub fn latest(&self) -> &Version {
        self.all.last().expect("Unable to get lastest version")
    }
}

impl Version {
    pub fn new(
        id: Id,
        number: VersionNumber,
        commit: Hash,
        zip_hash: Hash,
        created_timestamp: Timestamp,
    ) -> Version {
        Version {
            id,
            number,
            commit,
            zip_hash,
            created_timestamp,
        }
    }
}
