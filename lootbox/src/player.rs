use std::collections::{HashMap, HashSet};
use crate::currency::Currency;
use crate::sticker::Sticker;

#[derive(Clone, Debug, Default)]
pub struct Statistics {
    pub resets: u32,
    pub currencies_unlocked: u32,
    pub currencies_maxed: HashSet<Currency>,
    pub stickers_collected: u32,
}

impl Statistics {
    pub fn no_of_currencies_maxed(&self) -> u32 {
        self.currencies_maxed.len() as u32
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    currencies: HashMap<Currency, u32>,
    stickers: Vec<Sticker>,
    pub statistics: Statistics
}

impl Player {

    pub fn new(name: String) -> Player {
        Player { name, currencies: HashMap::new(), stickers: Vec::new(), statistics: Statistics::default() }
    }

    pub fn add_sticker(&mut self, sticker: Sticker) {
        if !self.stickers.contains(&sticker) {
            self.stickers.push(sticker);
            self.stickers.sort();
            self.statistics.stickers_collected += 1;
        }
    }

    pub fn add_currency(&mut self, currency: &Currency, amount: u32) {
        if !self.currencies.contains_key(currency) {
            self.currencies.insert((*currency).clone(), amount);
            self.statistics.currencies_unlocked += 1;
        }
        else {
            let overflow_check = amount.overflowing_add(self.currencies[currency]);
            if overflow_check.1 {
                self.currencies.insert((*currency).clone(), u32::MAX);
                self.statistics.currencies_maxed.insert((*currency).clone());
            }
            else {
                self.currencies.insert((*currency).clone(), overflow_check.0);
            }
        }
    }
}