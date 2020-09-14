use std::convert::TryFrom;
use super::targets_model::{Target, Targets};

pub fn create_targets(targets: Vec<&str>) -> Result<Targets, &'static str> {
    let mut new_targets: Vec<Target> = Vec::new();

    for target in targets {
        match Target::try_from(target) {
            Ok(i) => new_targets.push(i),
            Err(_) => return Err("Invalid target"),
        }
    }

    Ok(Targets::new(new_targets))
}

impl TryFrom<&str> for Target {
    type Error = &'static str;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "bs1" => Ok(Target::BS1),
            "bs2" => Ok(Target::BS2),
            "bs2e" => Ok(Target::BS2E),
            "bs2sx" => Ok(Target::BS2SX),
            "bs2p24" => Ok(Target::BS2P24),
            "bs2p40" => Ok(Target::BS2P40),
            "bs2pe" => Ok(Target::BS2PE),
            "bs2px" => Ok(Target::BS2PX),
            "sx" => Ok(Target::SX),
            "p1" => Ok(Target::P1),
            "p2" => Ok(Target::P2),
            _ => Err("Not a valid target"),
        }
    }
}
