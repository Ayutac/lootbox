use std::collections::HashMap;
use crate::currency::Currency;
use crate::sticker::Sticker;

#[derive(Clone, Debug, Default)]
pub struct Statistics {
    pub resets: u32,
    pub currencies_unlocked: u32,
    pub stickers_collected: u32,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub currencies: HashMap<Currency, u32>,
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
}