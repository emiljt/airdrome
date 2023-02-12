use actix_web::web::Bytes;
use awc::Client;
use base64;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

const B2_APPLICATION_KEY_ID_ENV: &str = "B2_APPLICATION_KEY_ID";
const B2_APPLICATION_TOKEN_ENV: &str = "B2_APPLICATION_TOKEN";
const B2_API_VERSION: &str = "2";

pub async fn authorize_account() -> Result<Session, &'static str> {
    let client = get_web_client(None);
    let application_id =
        env::var(B2_APPLICATION_KEY_ID_ENV).expect("B2 application key id not set");
    let application_key = env::var(B2_APPLICATION_TOKEN_ENV).expect("B2 application key not set");
    let auth_header_value = format!("{}:{}", application_id, application_key);

    let mut response = match client
        .get("https://api.backblazeb2.com/b2api/v2/b2_authorize_account")
        .insert_header((
            "Authorization",
            format!("Basic {}", base64::encode(&auth_header_value.into_bytes())),
        ))
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            panic!("Unable to authorize storage service")
        }
    };

    let token = response
        .json::<Session>()
        .await
        .expect("Unable to parse token");

    Ok(token)
}

pub async fn get_download_authorization(
    session: &Session,
    bucket_id: &str,
    prefix: &str,
    duration: u16,
) -> Result<DownloadToken, &'static str> {
    let client = get_web_client(None);

    let mut response = client
        .post(format!(
            "{}/b2api/v{}/b2_get_download_authorization",
            session.apiUrl, B2_API_VERSION
        ))
        .insert_header(("Authorization", &*session.authorizationToken))
        .send_json::<DownloadAuthorizationRequest>(&DownloadAuthorizationRequest {
            bucketId: bucket_id.to_string(),
            fileNamePrefix: prefix.to_string(),
            validDurationInSeconds: duration,
            b2ContentDisposition: None,
            b2ContentLanguage: None,
            b2Expires: None,
            b2CacheControl: None,
            b2ContentEncoding: None,
            b2ContentType: None,
        })
        .await
        .expect("Unable to get download authorization");

    let token = response
        .json::<DownloadToken>()
        .await
        .expect("Unable to parse download token");

    Ok(token)
}

pub async fn get_upload_url(
    session: Session,
    bucket_id: &str,
) -> Result<UploadInformation, &'static str> {
    let client = get_web_client(None);

    let mut response = client
        .post(format!(
            "{}/b2api/v{}/b2_get_upload_url",
            session.apiUrl, B2_API_VERSION
        ))
        .insert_header(("Authorization", session.authorizationToken))
        .send_json::<UploadUrlRequest>(&UploadUrlRequest {
            bucketId: bucket_id.to_string(),
        })
        .await
        .expect("Unable to get upload url");

    let info = response
        .json::<UploadInformation>()
        .await
        .expect("Unable to parse upload url");

    Ok(info)
}

pub async fn get_file_info(
    session: Session,
    file_id: &str,
) -> Result<FileInformation, &'static str> {
    let client = get_web_client(None);

    let mut response = client
        .post(format!(
            "{}/b2api/v{}/b2_get_file_info",
            session.apiUrl, B2_API_VERSION
        ))
        .insert_header(("Authorization", session.authorizationToken))
        .send_json::<FileInformationRequest>(&FileInformationRequest {
            fileId: file_id.to_string(),
        })
        .await
        .expect("Unable to get file information");

    let info = response
        .json::<FileInformation>()
        .await
        .expect("Unable to parse file information");

    Ok(info)
}

pub async fn upload_file(
    upload_info: UploadInformation,
    file_path: &str,
    file_name: Option<&str>,
    content_type: Option<&str>,
) -> Result<FileInformation, &'static str> {
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");
    let client = get_web_client(Some(300));
    let file_path = Path::new(file_path);
    let mut file = File::open(file_path).expect("Unable to open file for upload");
    let file_name = file_name.unwrap_or(
        file_path
            .file_name()
            .expect("Invalid file name")
            .to_str()
            .expect("Failed to parse file name"),
    );
    let content_type = content_type.unwrap_or("b2/x-auto");
    let mut hasher = Sha1::new();
    let mut file_buffer = Vec::new();

    file.read_to_end(&mut file_buffer)
        .expect("Unable to read file for hashing");
    hasher.update(&file_buffer);

    let file_hash = hasher.finalize();

    let mut response = client
        .post(upload_info.uploadUrl)
        .insert_header(("Authorization", upload_info.authorizationToken))
        .insert_header(("X-Bz-File-Name", file_name))
        .insert_header(("Content-Type", content_type))
        .insert_header((
            "Content-Length",
            file.metadata().expect("Unable to get file metadata").len(),
        ))
        .insert_header(("X-Bz-Content-Sha1", format!("{:x}", file_hash)))
        .insert_header(("X-Bz-Info-uploadSource", crate_name))
        .insert_header(("X-Bz-Info-apiVersion", crate_version))
        .send_body(Bytes::from(file_buffer))
        .await
        .expect("Unable to upload file");

    let info = response
        .json::<FileInformation>()
        .await
        .expect("Unable to parse file information");

    Ok(info)
}

pub async fn download_file(
    session: Session,
    bucket_id: &str,
    target_path: &str,
    file_name: &str,
) -> Result<(), String> {
    let client = get_web_client(None);

    let mut response = client
        .get(format!(
            "{}/file/{}/{}",
            session.apiUrl,
            bucket_id.to_string(),
            file_name.to_string(),
        ))
        .insert_header(("Authorization", session.authorizationToken))
        .send()
        .await
        .expect("Unable to send file download request");

    println!("{:?}", response);
    if !response.status().is_success() {
        return Err(response
            .json::<String>()
            .await
            .expect("Unable to retrieve request body"));
    }

    File::create(format!("{}/{}", target_path, file_name))
        .expect("Unable to create file")
        .write_all(
            &response
                .body()
                .await
                .expect("Unable to retrieve response body"),
        )
        .expect("Unable to write file");

    Ok(())
}

fn get_web_client(timeout: Option<u16>) -> Client {
    let timeout = match timeout {
        Some(t) => Duration::from_secs(t.into()),
        None => Duration::from_secs(30),
    };
    // let connector = Connector::new().timeout(Duration::from_secs(20)).finish();

    Client::builder()
        // .connector(connector)
        .timeout(timeout)
        .finish()
}

#[derive(Debug, Deserialize)]
pub struct Session {
    accountId: String,
    pub authorizationToken: String,
    allowed: TokenPermissions,
    apiUrl: String,
    pub downloadUrl: String,
    recommendedPartSize: i32,
    absoluteMinimumPartSize: i32,
}

#[derive(Deserialize)]
pub struct DownloadToken {
    pub authorizationToken: String,
    bucketId: String,
    fileNamePrefix: String,
}

#[derive(Debug, Deserialize)]
pub struct FileInformation {
    accountId: String,
    action: FileAction,
    bucketId: String,
    contentLength: i32,
    pub contentSha1: String,
    contentMd5: Option<String>,
    contentType: String,
    fileId: String,
    fileInfo: CustomFileInformation,
    fileName: String,
    uploadTimestamp: i64,
}

#[derive(Debug, Deserialize)]
pub struct UploadInformation {
    bucketId: String,
    uploadUrl: String,
    authorizationToken: String,
}

#[derive(Debug, Deserialize)]
struct TokenPermissions {
    capabilities: Vec<String>,
    bucketId: Option<String>,
    bucketName: Option<String>,
    namePrefix: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CustomFileInformation {
    uploadSource: Option<String>,
    apiVersion: Option<String>,
}

#[derive(Debug, Serialize)]
struct DownloadAuthorizationRequest {
    bucketId: String,
    fileNamePrefix: String,
    validDurationInSeconds: u16,
    b2ContentDisposition: Option<String>,
    b2ContentLanguage: Option<String>,
    b2Expires: Option<String>,
    b2CacheControl: Option<String>,
    b2ContentEncoding: Option<String>,
    b2ContentType: Option<String>,
}

#[derive(Debug, Serialize)]
struct FileInformationRequest {
    fileId: String,
}

#[derive(Debug, Serialize)]
struct UploadUrlRequest {
    bucketId: String,
}

#[derive(Debug, Deserialize)]
enum FileAction {
    start,
    upload,
    hide,
    folder,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn authorization() {
        let token = authorize_account().await;

        assert!(token.is_ok());
    }

    #[actix_rt::test]
    async fn download_authorization() {
        let bucket_id = env::var("B2_TEST_BUCKET_ID").expect("B2 test bucket Id not set");
        let session = authorize_account()
            .await
            .expect("Unable to authenticate with storage service");
        let token = get_download_authorization(&session, &bucket_id, "", 300).await;

        assert!(token.is_ok());
    }

    #[actix_rt::test]
    async fn file_information() {
        let file_id = env::var("B2_TEST_FILE_ID").expect("B2 test file Id not set");
        let session = authorize_account()
            .await
            .expect("Unable to authenticate with storage service");

        let info = get_file_info(session, &file_id).await;

        assert!(info.is_ok());
    }

    #[actix_rt::test]
    async fn upload_url() {
        let bucket_id = env::var("B2_TEST_BUCKET_ID").expect("B2 test bucket Id not set");
        let session = authorize_account()
            .await
            .expect("Unable to authenticate with storage service");

        let url = get_upload_url(session, &bucket_id).await;

        assert!(url.is_ok());
    }

    #[actix_rt::test]
    async fn upload() {
        let bucket_id = env::var("B2_TEST_BUCKET_ID").expect("B2 test bucket Id not set");
        let session = authorize_account()
            .await
            .expect("Unable to authenticate with storage service");

        let url = get_upload_url(session, &bucket_id)
            .await
            .expect("Failed to get upload URL");
        let info = upload_file(url, "./Cargo.toml", None, None).await;

        assert!(info.is_ok());
    }

    #[actix_rt::test]
    async fn download() {
        let bucket_name = env::var("B2_TEST_BUCKET_NAME").expect("B2 test bucket name not set");
        let session = authorize_account()
            .await
            .expect("Unable to authenticate with storage service");

        let result = download_file(session, &bucket_name, "/tmp", "Cargo.toml").await;

        assert!(result.is_ok());
    }
}
