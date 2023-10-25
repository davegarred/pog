use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Wager {
    pub offering: String,
    pub accepting: String,
    pub wager: String,
    pub outcome: String,
}

impl Display for Wager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} vs {}, {} - {}",
            self.offering, self.accepting, self.wager, self.outcome
        )
    }
}
