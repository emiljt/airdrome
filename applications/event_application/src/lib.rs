use std::{sync, thread, time};

pub fn create_event_thread() -> sync::mpsc::Sender<Event> {
    let (tx, rx) = sync::mpsc::channel::<Event>();

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(1));

        match rx.try_recv() {
            Ok(event) => println!("Event: {:?}", event),
            Err(_) => (),
        }
    });

    tx
}

#[derive(Debug)]
pub enum Event {
    ServerStarted,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
