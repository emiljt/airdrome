use std::convert::TryFrom;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize};
use chrono::{NaiveDateTime};

pub async fn get_objects(query: web::Query<ObjectsQuery>) -> impl Responder {
    let mut microcontrollers: Vec<Microcontroller> = Vec::new();
    let mut languages: Vec<Language> = Vec::new();
    let keywords: Vec<&str> = match &query.keywords {
        Some(list) => list.split(",").collect(),
        None => Vec::new(),
    };

    match &query.microcontrollers {
        Some(list) => {
            let list = list.split(",");

            for item in list {
                match Microcontroller::try_from(item) {
                    Ok(i) => microcontrollers.push(i),
                    Err(e) => {
                        return HttpResponse::BadRequest()
                            .body("Invalid microcontroller");
                    },
                }
            }
        },
        _ => (),
    };

    match &query.languages {
        Some(list) => {
            let list = list.split(",");

            for item in list {
                match Language::try_from(item) {
                    Ok(i) => languages.push(i),
                    Err(_) => {
                        return HttpResponse::BadRequest()
                            .body("Invalid microcontroller");
                    },
                }
            }
        },
        _ => (),
    };

    let created_date: Option<NaiveDateTime> = match &query.created {
        Some(date) => {
            match NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S") {
                Ok(parsed_date) => Some(parsed_date),
                _ => {
                    return HttpResponse::BadRequest()
                        .body("Invalid created date, expected format: yyyy-mm-ddThh:mm:ss");
                },
            }
        },
        None => None,
    };

    let updated_date: Option<NaiveDateTime> = match &query.updated {
        Some(date) => {
            match NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S") {
                Ok(parsed_date) => Some(parsed_date),
                _ => {
                    return HttpResponse::BadRequest()
                        .body("Invalid created date, expected format: yyyy-mm-ddThh:mm:ss");
                },
            }
        },
        None => None,
    };

    HttpResponse::Ok()
        .body(format!("
            limit: {:?},
            created: {:?},
            updated: {:?},
            name: {:?},
            microcontroller: {:?},
            language: {:?},
            keyword: {:?},
            category: {:?}", query.limit, created_date, updated_date, query.name,
            microcontrollers, languages, keywords, query.category)
        )
}

pub async fn get_object(id: web::Path<String>) -> HttpResponse {
    let guid: uuid::Uuid = match uuid::Uuid::parse_str(&id) {
        Ok(i) => i,
        Err(_) => {
            return HttpResponse::BadRequest()
            .body(format!("Invalid guid given: {}", &id))
        },
    };

    HttpResponse::Ok()
        .body(format!("{}", guid))
}

#[derive(Deserialize)]
pub struct ObjectsQuery {
    limit: Option<u32>,
    created: Option<String>,
    updated: Option<String>,
    name: Option<String>,
    microcontrollers: Option<String>,
    languages: Option<String>,
    keywords: Option<String>,
    category: Option<Category>,
}

#[derive(Debug, Deserialize)]
enum Microcontroller {
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

impl TryFrom<&str> for Microcontroller {
    type Error = &'static str;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "bs1" => Ok(Microcontroller::BS1),
            "bs2" => Ok(Microcontroller::BS2),
            "bs2e" => Ok(Microcontroller::BS2E),
            "bs2sx" => Ok(Microcontroller::BS2SX),
            "bs2p24" => Ok(Microcontroller::BS2P24),
            "bs2p40" => Ok(Microcontroller::BS2P40),
            "bs2pe" => Ok(Microcontroller::BS2PE),
            "bs2px" => Ok(Microcontroller::BS2PX),
            "sx" => Ok(Microcontroller::SX),
            "p1" => Ok(Microcontroller::P1),
            "p2" => Ok(Microcontroller::P2),
            _ => Err("Not a valid microcontroller"),
        }
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
enum Category {
}