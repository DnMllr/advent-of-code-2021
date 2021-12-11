#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Delimeter {
    Paren,
    Square,
    Curly,
    Angle,
}

impl std::fmt::Display for Delimeter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl Delimeter {
    pub fn syntax_score(self) -> usize {
        match self {
            Delimeter::Paren => 3,
            Delimeter::Square => 57,
            Delimeter::Curly => 1197,
            Delimeter::Angle => 25137,
        }
    }

    pub fn autocomplete_score(self) -> usize {
        match self {
            Delimeter::Paren => 1,
            Delimeter::Square => 2,
            Delimeter::Curly => 3,
            Delimeter::Angle => 4,
        }
    }
}
