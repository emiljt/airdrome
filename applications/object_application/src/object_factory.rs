use super::object_model::Object;
use super::name_factory;

pub fn create_object(name: &str) -> Result<Object, &'static str> {
    let new_name = name_factory::create_name(name)?;

    Ok(Object::new(new_name))
}
