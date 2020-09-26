use std::fmt;

#[derive(Clone)]
pub struct Targets {
    pub value: Vec<Target>,
}

#[derive(Clone, Debug)]
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
            value: targets,
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
