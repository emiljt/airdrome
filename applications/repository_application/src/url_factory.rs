use super::url_model::Url;

pub fn create_url(new_url: &str) -> Result<Url, &'static str> {
    let new_url = url::Url::parse(new_url).expect("Unable to parse url");

    Url::new(new_url.as_str())
}
