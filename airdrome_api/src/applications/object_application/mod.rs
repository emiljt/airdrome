mod description_factory;
mod description_model;
mod hash_factory;
mod hash_model;
mod id_factory;
mod id_model;
mod filename_model;
mod filename_factory;
mod languages_factory;
mod languages_model;
mod name_factory;
mod name_model;
mod object_factory;
mod object_model;
mod object_repository;
mod targets_factory;
mod targets_model;
mod timestamp_factory;
mod timestamp_model;
mod version_number_factory;
mod version_number_model;
mod versions_factory;
mod versions_model;
mod versions_repository;

use crate::services::files_service;
use crate::services::storage_service;
use log::{debug, error, info, warn};
use sha1::{Digest, Sha1};
use std::convert::From;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub async fn add_new_object(
    // db_connection: sqlx::pool::PoolConnection<sqlx::MySql>,
    db_pool: &sqlx::Pool<sqlx::MySql>,
    name: &str,
    description: &str,
    languages: Vec<&str>,
    targets: Vec<&str>,
    path: &Path,
    initial_version_number: Option<&str>,
    initial_version_commit: Option<&str>,
) -> Result<Object, &'static str> {
    let filename = match filename_factory::create_filename(name, initial_version_number) {
        Ok(f) => f,
        Err(_) => return Err("Unable to create filename"),
    };

    // Because the modified timestamp on files effects the hash of the zip file,
    // we have to pass in what will become the version.created_timestamp so that
    // it can be used to creating the zip file as well as in the hash comparison
    // of new zip files
    let created_timestamp =
        timestamp_factory::create_timestamp(None).expect("Unable to create created timestamp");

    let object_file = match compress_object(&filename.value, path, Some(&created_timestamp.value)).await {
        Ok(f) => f,
        Err(e) => {
            error!("Error creating compressed object file");
            return Err("Unable to compress object files")
        },
    };

    let initial_version = versions_factory::create_version(
        initial_version_number,
        initial_version_commit.unwrap_or(""),
        &object_file.zip_hash,
        Some(&created_timestamp.value),
    )?;

    let new_object = object_factory::create_object(
        name,
        description,
        languages,
        targets,
        vec![initial_version.clone()],
    )?;

    // upload_object(
    //     &object_file,
    //     &new_object.id.value,
    //     &initial_version.id.value,
    // )
    // .await?;

    object_repository::save_object(db_pool, &new_object).await?;

    Ok(Object::from_object_model(new_object))
}

pub async fn update_object(
    db_pool: &sqlx::Pool<sqlx::MySql>,
    id: &str,
    description: &str,
    languages: Vec<&str>,
    targets: Vec<&str>,
    path: &Path,
    new_version_number: Option<&str>,
    new_version_commit: Option<&str>,
) -> Result<Object, &'static str> {
    let object =
        match object_repository::read_object(db_pool, &id_factory::create_id(Some(&id)).expect(""))
            .await
        {
            Ok(o) => o,
            Err(_) => return Err("Unable to load existing object"),
        };

    let filename = match filename_factory::create_filename(
        &object.name.value,
        new_version_number,
    ) {
        Ok(f) => f,
        Err(_) => return Err("Unable to create filename"),
    };

    // Because the modified timestamp on files effects the hash of the zip file,
    // we need to use the same modified timestamp as the existing version so
    // that the zip hash comparison works
    let comparison_object_file = compress_object(
        &filename.value,
        path,
        Some(&object.versions.latest().created_timestamp.value),
    )
    .await
    .expect("Unable to create compress object file");

    info!(
        "Latest version hash: {}, new version hash: {}",
        object.versions.latest().zip_hash.value,
        comparison_object_file.zip_hash
    );

    if object.versions.latest().zip_hash.value == comparison_object_file.zip_hash {
        info!("Object zip up to date");

        let new_object = object_factory::create_object(
            &object.name.value,
            description,
            languages.clone(),
            targets.clone(),
            object.versions.all.clone(),
        )?;

        object_repository::save_object(db_pool, &new_object).await?;

        Ok(Object::from_object_model(new_object))
    } else {
        info!("Found new version of object");

        // Because the modified timestamp on files effects the hash of the zip file,
        // we have to pass in what will become the version.created_timestamp so that
        // it can be used to creating the zip file as well as in the hash comparison
        // of new zip files
        let created_timestamp =
            timestamp_factory::create_timestamp(None).expect("Unable to create created timestamp");

        let new_object_file = compress_object(&filename.value, path, Some(&created_timestamp.value))
            .await
            .expect("Unable to create compress object file");

        let new_version = versions_factory::create_version(
            new_version_number,
            new_version_commit.unwrap_or(""),
            &new_object_file.zip_hash,
            Some(&created_timestamp.value),
        )?;

        let mut new_versions = object.versions.all.clone();
        new_versions.push(new_version);

        let new_object = object_factory::create_object(
            &object.name.value,
            description,
            languages,
            targets,
            new_versions,
        )?;

        object_repository::save_object(db_pool, &new_object).await?;

        // upload_object(
        //     &new_object_file,
        //     &new_object.id.value,
        //     &new_object.versions.latest().id.value,
        // )
        // .await?;

        Ok(Object::from_object_model(new_object))
    }
}

pub async fn find_object(
    // db_connection: sqlx::pool::PoolConnection<sqlx::MySql>,
    db_pool: &sqlx::Pool<sqlx::MySql>,
    id: &str,
) -> Result<Object, &'static str> {
    let id = id_factory::create_id(Some(id)).expect("");

    match object_repository::read_object(db_pool, &id).await {
        Ok(object) => Ok(Object::from_object_model(object)),
        Err(_) => Err("No object found"),
    }
}

pub async fn search_objects(
    // db_connection: sqlx::pool::PoolConnection<sqlx::MySql>,
    db_pool: &sqlx::Pool<sqlx::MySql>,
    name: Option<&str>,
    targets: Option<Vec<&str>>,
    languages: Option<Vec<&str>>,
    keywords: Option<&str>,
    categories: Option<Vec<&str>>,
    created: Option<&str>,
    updated: Option<&str>,
) -> Vec<Object> {
    let mut results = Vec::new();

    match object_repository::read_objects(db_pool, name, targets, languages, keywords).await {
        Ok(objects) => {
            for result in objects {
                results.push(Object::from_object_model(result));
            }

            results
        }
        Err(e) => {
            error!("Error searching objects: {}", e);
            Vec::new()
        }
    }
}

pub async fn find_version(
    db_pool: &sqlx::Pool<sqlx::MySql>,
    object_id: &str,
    version_id: &str,
) -> Result<Version, &'static str> {
    let object_id = id_factory::create_id(Some(object_id)).expect("Invalid object Id");
    let version_id = id_factory::create_id(Some(version_id)).expect("Invalid version Id");

    // Find given object
    let object = match object_repository::read_object(db_pool, &object_id).await {
        Ok(o) => o,
        Err(_) => return Err("No object found"),
    };

    // Make sure requested version exists
    let valid_version = object
        .versions
        .all
        .binary_search_by(|v| v.id.value.cmp(&version_id.value));

    match valid_version {
        Ok(i) => Ok(Version::from(&object.versions.all[i])),
        Err(_) => Err("No matching version found"),
    }
}

pub async fn get_compressed_object(
    db_pool: &sqlx::Pool<sqlx::MySql>,
    object_id: &str,
    version_id: &str,
) -> Result<String, &'static str> {
    let objects_path = env::var("OBJECTS_PATH").expect("OBJECTS_PATH environment variable not set");
    let mut version_path = PathBuf::from(objects_path);

    let object = match object_repository::read_object(
            db_pool,
            &id_factory::create_id(Some(&object_id)).expect("")
        ).await
    {
            Ok(o) => o,
            Err(_) => return Err("Unable to load existing object"),
    };

    let version = match find_version(db_pool, object_id, version_id).await {
        Ok(v) => v,
        Err(_) => return Err("Unable to find version"),
    };

    let filename = match filename_factory::create_filename(
        &object.name.value,
        Some(&version.number)
    ) {
        Ok(f) => f,
        Err(_) => return Err("Unable to create filename"),
    };

    version_path.set_file_name(&filename.value);
    version_path.set_extension("zip");

    if !version_path.exists() {
        download_object(object_id, version_id)
            .await
            .expect("Unable to download version");
    }

    Ok(format!("/static/objects/{}.zip", &filename.value))
}

async fn compress_object(
    filename: &str,
    path: &Path,
    modified_timestamp: Option<&str>,
) -> Result<CompressedObject, &'static str> {
    let objects_path = env::var("OBJECTS_PATH").expect("OBJECTS_PATH environment variable not set");
    let objects_destination = Path::new(&objects_path);

    match create_dir_all(objects_destination) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            return Err("Unable to create objects directory");
        }
    }

    let version_zip_path = match files_service::create_zip_file(
        path,
        objects_destination,
        &format!("{}.zip", filename),
        modified_timestamp,
    ) {
        Ok(f) => f,
        Err(e) => {
            error!("{}", e);
            return Err("Unable to create zip file");
        }
    };

    let mut zip_file =
        File::open(version_zip_path.as_path()).expect("Unable to open file for upload");
    let mut hasher = Sha1::new();
    let mut file_buffer = Vec::new();

    zip_file
        .read_to_end(&mut file_buffer)
        .expect("Unable to read file for hashing");

    hasher.update(&file_buffer);

    let zip_hash = hasher.finalize();

    let mut hash = String::new();

    for i in zip_hash.iter() {
        hash.push_str(&format!("{:02x?}", i));
    }

    Ok(CompressedObject {
        zip_path: version_zip_path
            .to_str()
            .expect("Unable to convert zip path to string")
            .to_string(),
        zip_hash: hash,
    })
}

async fn upload_object(
    target: &CompressedObject,
    object_id: &str,
    version_id: &str,
) -> Result<(), &'static str> {
    let bucket_id = env::var("B2_BUCKET_ID").expect("B2_BUCKET_ID environment variable not set");
    let storage_session = storage_service::authorize_account().await?;
    let upload_url = storage_service::get_upload_url(storage_session, &bucket_id).await?;

    let upload = storage_service::upload_file(
        upload_url,
        &target.zip_path,
        Some(&format!("objects/{}/{}.zip", object_id, version_id)),
        None,
    )
    .await?;

    if upload.contentSha1 != target.zip_hash {
        return Err("Version zip hash didn't match the file upload hash");
    }

    Ok(())
}

async fn download_object(object_id: &str, version_id: &str) -> Result<(), &'static str> {
    let bucket_id = env::var("B2_BUCKET_ID").expect("B2_BUCKET_ID environment variable not set");
    let bucket_name =
        env::var("B2_BUCKET_NAME").expect("B2_BUCKET_NAME environment variable not set");
    let objects_path = env::var("OBJECTS_PATH").expect("OBJECTS_PATH environment variable not set");

    let session = storage_service::authorize_account()
        .await
        .expect("Unable to authenticate with storage service");

    // let download_token =
    //     storage_service::get_download_authorization(&session, &bucket_id, "objects", 300)
    //         .await
    //         .expect("Unable to get download authorization");
    storage_service::download_file(session, &bucket_id, &objects_path, object_id);

    Ok(())
}

pub struct Object {
    pub id: String,
    pub name: String,
    pub targets: Vec<Target>,
    pub languages: Vec<Language>,
    pub versions: Vec<Version>,
}

impl Object {
    fn from_object_model(object_model: object_model::Object) -> Object {
        let mut targets: Vec<Target> = Vec::new();
        let mut languages: Vec<Language> = Vec::new();
        let mut versions: Vec<Version> = Vec::new();

        for target in object_model.targets.value {
            targets.push(Target::from(target));
        }

        for language in object_model.languages.value {
            languages.push(Language::from(language));
        }

        for version in object_model.versions.all {
            versions.push(Version::from(&version));
        }

        Object {
            id: object_model.id.value,
            name: object_model.name.value,
            targets: targets,
            languages: languages,
            versions: versions,
        }
    }
}

#[derive(Debug)]
pub enum Target {
    BS1,
    BS2,
    BS2E,
    BS2SX,
    BS2P24,
    BS2P40,
    BS2PE,
    BS2PX,
    SX,
    P1,
    P2,
}

impl From<targets_model::Target> for Target {
    fn from(item: targets_model::Target) -> Target {
        match item {
            targets_model::Target::BS1 => Target::BS1,
            targets_model::Target::BS2 => Target::BS2,
            targets_model::Target::BS2E => Target::BS2E,
            targets_model::Target::BS2SX => Target::BS2SX,
            targets_model::Target::BS2P24 => Target::BS2P24,
            targets_model::Target::BS2P40 => Target::BS2P40,
            targets_model::Target::BS2PE => Target::BS2PE,
            targets_model::Target::BS2PX => Target::BS2PX,
            targets_model::Target::SX => Target::SX,
            targets_model::Target::P1 => Target::P1,
            targets_model::Target::P2 => Target::P2,
        }
    }
}

#[derive(Debug)]
pub enum Language {
    Spin,
    Spin2,
    Pasm,
    Pasm2,
    C,
    Basic,
    Forth,
    Python,
}

impl From<languages_model::Language> for Language {
    fn from(item: languages_model::Language) -> Language {
        match item {
            languages_model::Language::Spin => Language::Spin,
            languages_model::Language::Spin2 => Language::Spin2,
            languages_model::Language::Pasm => Language::Pasm,
            languages_model::Language::Pasm2 => Language::Pasm2,
            languages_model::Language::C => Language::C,
            languages_model::Language::Basic => Language::Basic,
            languages_model::Language::Forth => Language::Forth,
            languages_model::Language::Python => Language::Python,
        }
    }
}

pub struct Version {
    pub id: String,
    pub number: String,
    pub commit: String,
    pub created_timestamp: String,
}

impl From<&versions_model::Version> for Version {
    fn from(item: &versions_model::Version) -> Version {
        Version {
            id: item.id.value.clone(),
            number: item.number.value.clone(),
            commit: item.commit.value.clone(),
            created_timestamp: item.created_timestamp.value.clone(),
        }
    }
}

struct CompressedObject {
    zip_path: String,
    zip_hash: String,
}

pub struct DownloadUri {
    pub token: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
