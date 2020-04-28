use super::url_model::Url;

fn create_url(new_url: &str) -> Result(Url, &'static str) {
    let new_url = url::Url::parse(new_url)?;

    Url::new(new_address: new_url.as_str())
}
