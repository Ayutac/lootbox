use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use crate::currency::Currency;

#[derive(Clone, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub enum LootEntry {
    Sticker(u32),
    Money(Currency, u32)
}

impl Display for LootEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let LootEntry::Money(currency, amount) = self {
            write!(f, "{} {}", amount, currency)
        }
        else if let LootEntry::Sticker(amount) = self {
            write!(f, "{} sticker", amount)
        }
        else { unreachable!() }
    }
}

#[derive(Eq)]
pub struct LootTable(Vec<(LootEntry, u8)>);

impl Clone for LootTable {
    fn clone(&self) -> Self {
        LootTable(self.0.to_vec())
    }
}

impl Debug for LootTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut comma_separated = String::new();
        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str("(");
            comma_separated.push_str(&num.0.to_string());
            comma_separated.push_str(", ");
            comma_separated.push_str(&num.1.to_string());
            comma_separated.push_str("), ");
        }
        comma_separated.push_str("(");
        comma_separated.push_str(&self.0[self.0.len() - 1].0.to_string());
        comma_separated.push_str(", ");
        comma_separated.push_str(&self.0[self.0.len() - 1].1.to_string());
        comma_separated.push_str(")");
        write!(f, "{}", comma_separated)
    }
}

impl PartialEq for LootTable {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        let mut sorted = self.clone();
        sorted.0.sort();
        let mut other_sorted = other.clone();
        other_sorted.0.sort();
        for i in 0..sorted.0.len() {
            if sorted.0[i] != other_sorted.0[i] {
                return false;
            }
        }
        true
    }
}

impl Hash for LootTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut sorted = self.clone();
        sorted.0.sort();
        for i in 0..sorted.0.len() {
            sorted.0[i].0.hash(state);
            sorted.0[i].1.hash(state);
        }
    }
}

impl LootTable {
    pub fn size(&self) -> u32 {
        let mut size = 0u32;
        for (_, amount) in &self.0 {
            size += *amount as u32;
        }
        size
    }

    pub fn select(&self, selection: u32) -> Option<LootEntry> {
        if selection >= self.size() {
            return None;
        }
        let mut index = 0u32;
        for (entry, amount) in &self.0 {
            index += *amount as u32;
            if selection < index {
                return Some((*entry).clone());
            }
        }
        return None;
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LootBox {
    pub name: String,
    pub rarity: u8,
    pub price: (Currency, u32),
    pub no_of_rewards: u8,
    pub loot_table: LootTable
}

impl PartialOrd for LootBox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // cmp rarity
        if self.rarity > other.rarity {
            Some(Ordering::Greater)
        }
        else if self.rarity < other.rarity {
            Some(Ordering::Less)
        }
        // cmp currency
        else if self.price.0 > other.price.0 {
            Some(Ordering::Greater)
        }
        else if self.price.0 < other.price.0 {
            Some(Ordering::Less)
        }
        // cmp price
        else if self.price.1 > other.price.1 {
            Some(Ordering::Greater)
        }
        else if self.price.1 < other.price.1 {
            Some(Ordering::Less)
        }
        else {
            self.name.partial_cmp(&other.name)
        }
    }
}