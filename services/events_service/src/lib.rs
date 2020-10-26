#[derive(Debug)]
pub enum Event {
    ServerStarted,
    ObexObjectAdded { name: String, path: String },
}
