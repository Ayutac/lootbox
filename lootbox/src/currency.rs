use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord)]
pub struct Currency {
    pub name: String,
    pub rarity: u8
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rarity > other.rarity {
            Some(Ordering::Greater)
        }
        else if self.rarity < other.rarity {
            Some(Ordering::Less)
        }
        else {
            Some(self.name.cmp(&other.name))
        }
    }
}