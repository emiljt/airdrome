use events_service::Event;
use obex::obex::Obex;
use std::path::Path;
use std::sync::mpsc;

pub fn sync(event_tx: mpsc::Sender<Event>, path: &str) {
    let path = Path::new(path);
    let obex = Obex::new(
        "",
        &path
            .to_str()
            .expect("Unable able to convert string to path"),
    )
    .expect("Unable create obex");

    for object in &obex.official_categories {}

    for object in &obex.community_categories {}

    for object in &obex.official_objects {
        event_tx
            .send(Event::ObexObjectAdded {
                name: object.name.to_string(),
                path: object.path.to_string(),
            })
            .expect("Error sending obexObjectAdded event");
    }

    for object in &obex.community_objects {
        event_tx
            .send(Event::ObexObjectAdded {
                name: object.name.to_string(),
                path: object.path.to_string(),
            })
            .expect("Error sending obexObjectAdded event");
    }
}

pub struct ObexObject {
    name: String,
    path: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
