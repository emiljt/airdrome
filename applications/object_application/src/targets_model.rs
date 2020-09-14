pub struct Targets {
    targets: Vec<Target>,
}

pub enum Target {
    BS1,
    BS2,
    BS2E,
    BS2SX,
    BS2P24,
    BS2P40,
    BS2PE,
    BS2PX,
    SX,
    P1,
    P2,
}

impl Targets {
    pub fn new(targets: Vec<Target>) -> Targets {
        Targets {
            targets: targets,
        }
    }
}
