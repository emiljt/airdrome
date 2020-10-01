use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::convert::TryFrom;
use std::sync::mpsc;

use object_application::Object;

pub async fn get_objects(
    db_pool: web::Data<MySqlPool>,
    event_tx: web::Data<mpsc::Sender<&str>>,
    query: web::Query<ObjectsQuery>,
) -> impl Responder {
    let mut targets: Vec<&str> = Vec::new();
    let mut languages: Vec<&str> = Vec::new();
    let mut categories: Vec<&str> = Vec::new();

    match &query.targets {
        Some(list) => {
            let list = list.split(",");

            for item in list {
                match Target::try_from(item) {
                    Ok(i) => targets.push(item),
                    Err(_) => {
                        return HttpResponse::BadRequest().body("Invalid target");
                    }
                }
            }
        }
        _ => (),
    };

    let targets = if targets.len() > 0 {
        Some(targets)
    } else {
        None
    };

    match &query.languages {
        Some(list) => {
            let list = list.split(",");

            for item in list {
                match Language::try_from(item) {
                    Ok(i) => languages.push(item),
                    Err(_) => {
                        return HttpResponse::BadRequest().body("Invalid language");
                    }
                }
            }
        }
        _ => (),
    };

    let languages = if languages.len() > 0 {
        Some(languages)
    } else {
        None
    };

    match &query.created {
        Some(date) => match NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S") {
            Ok(_) => (),
            _ => {
                return HttpResponse::BadRequest()
                    .body("Invalid created date, expected format: yyyy-mm-ddThh:mm:ss");
            }
        },
        None => (),
    };

    match &query.updated {
        Some(date) => match NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S") {
            Ok(_) => (),
            _ => {
                return HttpResponse::BadRequest()
                    .body("Invalid created date, expected format: yyyy-mm-ddThh:mm:ss");
            }
        },
        None => (),
    };

    match &query.categories {
        Some(list) => {
            let list = list.split(",");

            for item in list {
                match Category::try_from(item) {
                    Ok(i) => categories.push(item),
                    Err(_) => {
                        return HttpResponse::BadRequest().body("Invalid category");
                    }
                }
            }
        }
        _ => (),
    };

    let categories = if categories.len() > 0 {
        Some(categories)
    } else {
        None
    };

    let mut db_connection = match db_pool.acquire().await {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::ServiceUnavailable().finish(),
    };

    let objects = object_application::search_objects(
        &mut db_connection,
        query.name.as_deref(),
        targets,
        languages,
        query.keywords.as_deref(),
        categories,
        query.created.as_deref(),
        query.updated.as_deref(),
    )
    .await;

    let mut converted_objects: Vec<ObjectData> = Vec::new();

    for object in objects {
        match ObjectData::try_from(object) {
            Ok(o) => converted_objects.push(o),
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };
    }

    let json = match serde_json::to_string(&converted_objects) {
        Ok(r) => r,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().body(format!("{}", json))
}

pub async fn get_object(db_pool: web::Data<MySqlPool>, id: web::Path<String>) -> HttpResponse {
    let guid: uuid::Uuid = match uuid::Uuid::parse_str(&id) {
        Ok(i) => i,
        Err(_) => return HttpResponse::BadRequest().body(format!("Invalid guid given: {}", &id)),
    };

    let mut db_connection = match db_pool.acquire().await {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::ServiceUnavailable().finish(),
    };

    match object_application::find_object(&mut db_connection, &guid.to_string()).await {
        Ok(object) => {
            let data = match ObjectData::try_from(object) {
                Ok(object) => object,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            let json = match serde_json::to_string(&data) {
                Ok(r) => r,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            HttpResponse::Ok().body(format!("{}", json))
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[derive(Deserialize)]
pub struct ObjectsQuery {
    limit: Option<u32>,
    created: Option<String>,
    updated: Option<String>,
    name: Option<String>,
    targets: Option<String>,
    languages: Option<String>,
    keywords: Option<String>,
    categories: Option<String>,
}

#[derive(Serialize)]
struct ObjectData {
    guid: String,
    name: String,
    license: String,
    readme: String,
    website: String,
    documentation: String,
    authors: Vec<Author>,
    versions: Vec<Version>,
    targets: Vec<Target>,
    languages: Vec<Language>,
    stats: Vec<Stat>,
    categories: Vec<Category>,
}

impl TryFrom<Object> for ObjectData {
    type Error = &'static str;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let mut targets = Vec::new();
        let mut languages = Vec::new();

        for target in object.targets {
            targets.push(Target::into(Target::try_from(target)?));
        }

        for language in object.languages {
            languages.push(Language::into(Language::try_from(language)?));
        }

        Ok(ObjectData {
            guid: object.id,
            name: object.name,
            license: "".to_string(),
            readme: "".to_string(),
            website: "".to_string(),
            documentation: "".to_string(),
            authors: Vec::new(),
            versions: Vec::new(),
            targets: targets,
            languages: languages,
            stats: Vec::new(),
            categories: Vec::new(),
        })
    }
}

#[derive(Serialize)]
pub struct Author {
    name: String,
    email: String,
    website: String,
}

#[derive(Serialize)]
pub struct Version {
    number: String,
    created: String,
}

#[derive(Debug, Deserialize, Serialize)]
enum Target {
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

impl TryFrom<&str> for Target {
    type Error = &'static str;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "bs1" => Ok(Target::BS1),
            "bs2" => Ok(Target::BS2),
            "bs2e" => Ok(Target::BS2E),
            "bs2sx" => Ok(Target::BS2SX),
            "bs2p24" => Ok(Target::BS2P24),
            "bs2p40" => Ok(Target::BS2P40),
            "bs2pe" => Ok(Target::BS2PE),
            "bs2px" => Ok(Target::BS2PX),
            "sx" => Ok(Target::SX),
            "p1" => Ok(Target::P1),
            "p2" => Ok(Target::P2),
            _ => Err("Not a valid target"),
        }
    }
}

impl TryFrom<object_application::Target> for Target {
    type Error = &'static str;

    fn try_from(item: object_application::Target) -> Result<Self, Self::Error> {
        match item {
            object_application::Target::BS1 => Ok(Target::BS1),
            object_application::Target::BS2 => Ok(Target::BS2),
            object_application::Target::BS2E => Ok(Target::BS2E),
            object_application::Target::BS2SX => Ok(Target::BS2SX),
            object_application::Target::BS2P24 => Ok(Target::BS2P24),
            object_application::Target::BS2P40 => Ok(Target::BS2P40),
            object_application::Target::BS2PE => Ok(Target::BS2PE),
            object_application::Target::BS2PX => Ok(Target::BS2PX),
            object_application::Target::SX => Ok(Target::SX),
            object_application::Target::P1 => Ok(Target::P1),
            object_application::Target::P2 => Ok(Target::P2),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum Language {
    Spin,
    Spin2,
    Pasm,
    Pasm2,
    C,
    Basic,
    Forth,
    Python,
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "spin" => Ok(Language::Spin),
            "spin2" => Ok(Language::Spin2),
            "pasm" => Ok(Language::Pasm),
            "pasm2" => Ok(Language::Pasm2),
            "c" => Ok(Language::C),
            "basic" => Ok(Language::Basic),
            "forth" => Ok(Language::Forth),
            "python" => Ok(Language::Python),
            _ => Err("Not a valid language"),
        }
    }
}

impl TryFrom<object_application::Language> for Language {
    type Error = &'static str;

    fn try_from(item: object_application::Language) -> Result<Self, Self::Error> {
        match item {
            object_application::Language::Spin => Ok(Language::Spin),
            object_application::Language::Spin2 => Ok(Language::Spin2),
            object_application::Language::Pasm => Ok(Language::Pasm),
            object_application::Language::Pasm2 => Ok(Language::Pasm2),
            object_application::Language::C => Ok(Language::C),
            object_application::Language::Basic => Ok(Language::Basic),
            object_application::Language::Forth => Ok(Language::Forth),
            object_application::Language::Python => Ok(Language::Python),
        }
    }
}

#[derive(Serialize)]
pub struct Stat {
    name: String,
    value: String,
    updated: String,
}

#[derive(Debug, Deserialize, Serialize)]
enum Category {}

impl TryFrom<&str> for Category {
    type Error = &'static str;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            _ => Err("Not a valid category"),
        }
    }
}
