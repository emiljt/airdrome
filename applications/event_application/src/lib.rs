use events_service::Event;
use std::{sync, thread, time};

pub fn create_event_thread() -> sync::mpsc::Sender<Event> {
    let (tx, rx) = sync::mpsc::channel::<Event>();
    let event_tx = tx.clone();

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(1));

        match rx.try_recv() {
            Ok(e) => match e {
                Event::ServerStarted { temp_path } => {
                    obex_application::sync(event_tx.clone(), &temp_path)
                }
            },
            Err(_) => (),
        }
    });

    tx
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
