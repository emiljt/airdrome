mod object_model;
mod name_model;
mod object_factory;
mod name_factory;

use object_model::Object;

fn add_new_object(name: &str) -> Result<ObjectData, &'static str> {
    let new_object = object_factory::create_object(name)?;

    Ok(ObjectData::from_object(new_object))
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
