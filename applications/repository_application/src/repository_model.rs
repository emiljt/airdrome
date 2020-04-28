use super::type_model::Type;
use super::url_model::Url;

struct Repository {
    type: Type,
    url: Url,
}

impl Repository {
    fn new(type, url) -> Repository {
        Repository {
            type,
            url
        }
    }
}
