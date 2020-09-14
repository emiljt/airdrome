mod object_model;
mod name_model;
mod description_model;
mod languages_model;
mod targets_model;
mod object_factory;
mod name_factory;
mod description_factory;
mod languages_factory;
mod targets_factory;

use object_model::Object;

fn add_new_object(name: &str, description: &str, languages: Vec<&str>, targets: Vec<&str>)
    -> Result<ObjectData, &'static str> {
    let new_object = object_factory::create_object(name, description, languages, targets)?;

    Ok(ObjectData::from_object(new_object))
}

fn find_object(id: &str) -> Result<Object, &'static str> {
    Err("No object found")
}

fn search_objects(name: Option<&str>, targets: Option<&str>, languages: Option<&str>,
    keywords: Option<&str>, categories: Option<&str>, created: Option<&str>, updated: Option<&str>)
    -> Vec<Object> {
    Vec::new()
}

struct ObjectData {
    guid: String,
    name: String,
}

impl ObjectData {
    fn from_object(object: Object) -> ObjectData {
        ObjectData {
            guid: "".to_string(),
            name: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
