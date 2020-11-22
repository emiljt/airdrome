use actix_web::client::Client;
use actix_web::web::Bytes;
use base64;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const B2_APPLICATION_ID_KEY: &str = "B2_APPLICATION_ID";
const B2_APPLICATION_TOKEN_KEY: &str = "B2_APPLICATION_KEY";
const B2_API_VERSION: &str = "2";

pub async fn authorize_account() -> Result<Session, &'static str> {
    let client = Client::default();
    let application_id = env::var(B2_APPLICATION_ID_KEY).expect("B2 application id not set");
    let application_key = env::var(B2_APPLICATION_TOKEN_KEY).expect("B2 application key not set");
    let auth_header_value = format!("{}:{}", application_id, application_key);

    let mut response = client
        .get("https://api.backblazeb2.com/b2api/v2/b2_authorize_account")
        .header(
            "Authorization",
            format!("Basic {}", base64::encode(&auth_header_value.into_bytes())),
        )
        .send()
        .await
        .expect("Unable to authenticate with storage service");

    let token = response
        .json::<Session>()
        .await
        .expect("Unable to parse token");

    Ok(token)
}

pub async fn get_upload_url(
    session: Session,
    bucket_id: &str,
) -> Result<UploadInformation, &'static str> {
    let client = Client::default();

    let mut response = client
        .post(format!(
            "{}/b2api/v{}/b2_get_upload_url",
            session.apiUrl, B2_API_VERSION
        ))
        .header("Authorization", session.authorizationToken)
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
    let client = Client::default();

    let mut response = client
        .post(format!(
            "{}/b2api/v{}/b2_get_file_info",
            session.apiUrl, B2_API_VERSION
        ))
        .header("Authorization", session.authorizationToken)
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
    let client = Client::default();
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
        .header("Authorization", upload_info.authorizationToken)
        .header("X-Bz-File-Name", file_name)
        .header("Content-Type", content_type)
        .header(
            "Content-Length",
            file.metadata().expect("Unable to get file metadata").len(),
        )
        .header("X-Bz-Content-Sha1", format!("{:x}", file_hash))
        .header(
            "X-Bz-Info-uploadSource",
            env::var("CARGO_PKG_NAME").expect("Unable to get cargo package name"),
        )
        .header(
            "X-Bz-Info-apiVersion",
            env::var("CARGO_PKG_VERSION").expect("Unable to get cargo package version"),
        )
        .send_body(Bytes::from(file_buffer))
        .await
        .expect("Unable to upload file");

    let info = response
        .json::<FileInformation>()
        .await
        .expect("Unable to parse file information");

    Ok(info)
}

#[derive(Debug, Deserialize)]
pub struct Session {
    accountId: String,
    authorizationToken: String,
    allowed: TokenPermissions,
    apiUrl: String,
    downloadUrl: String,
    recommendedPartSize: i32,
    absoluteMinimumPartSize: i32,
}

#[derive(Debug, Deserialize)]
pub struct FileInformation {
    accountId: String,
    action: FileAction,
    bucketId: String,
    contentLength: i32,
    contentSha1: String,
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

    const TEST_BUCKET_ID: &str = "2cb36c59ee1dd5e87250061c";
    const TEST_FILE_ID: &str =
        "4_z2cb36c59ee1dd5e87250061c_f113a16a06639fd32_d20201119_m050453_c002_v0001149_t0012";

    #[actix_rt::test]
    async fn authorization() {
        let token = authorize_account().await;

        assert!(token.is_ok());
    }

    #[actix_rt::test]
    async fn file_information() {
        let session = authorize_account()
            .await
            .expect("Unable to authenticate with storage service");

        let info = get_file_info(session, TEST_FILE_ID).await;

        assert!(info.is_ok());
    }

    #[actix_rt::test]
    async fn upload_url() {
        let session = authorize_account()
            .await
            .expect("Unable to authenticate with storage service");

        let url = get_upload_url(session, TEST_BUCKET_ID).await;

        assert!(url.is_ok());
    }

    #[actix_rt::test]
    async fn upload() {
        let session = authorize_account()
            .await
            .expect("Unable to authenticate with storage service");

        let url = get_upload_url(session, TEST_BUCKET_ID)
            .await
            .expect("Failed to get upload URL");
        let info = upload_file(url, "./Cargo.toml", None, None).await;

        assert!(info.is_ok());
    }
}
