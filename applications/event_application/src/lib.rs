use std::{sync, thread, time};

pub fn create_event_thread() -> sync::mpsc::Sender<Event> {
    let (tx, rx) = sync::mpsc::channel::<Event>();

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(1));

        match rx.try_recv() {
            Ok(e) => match e {
                ServerStarted => println!("ServerStarted"),
                ObexObjectAdded => println!("ObexObjectAdded"),
            },
            Err(_) => (),
        }
    });

    tx
}

#[derive(Debug)]
pub enum Event {
    ServerStarted,
    ObexObjectAdded { name: String, path: String },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
