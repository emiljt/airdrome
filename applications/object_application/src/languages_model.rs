use std::fmt;

#[derive(Clone)]
pub struct Languages {
    pub value: Vec<Language>,
}

#[derive(Clone, Debug)]
pub enum Language {
    Spin,
    Spin2,
    Pasm,
    Pasm2,
    C,
    Basic,
    Forth,
    Python,
}

impl Languages {
    pub fn new(languages: Vec<Language>) -> Languages {
        Languages { value: languages }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
