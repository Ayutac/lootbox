use std::cmp::Ordering;
use crate::rng::Rng;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord)]
pub struct Currency {
    pub name: String,
    pub rarity: u8
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

impl Currency {
    fn generate(mut rng: Rng, rarity: u8) -> Self {
        Currency { name: rng.generate_word(), rarity }
    }
}