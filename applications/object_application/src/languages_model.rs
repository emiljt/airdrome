pub struct Languages {
    languages: Vec<Language>,
}

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
        Languages {
            languages: languages,
        }
    }
}
