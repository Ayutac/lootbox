use std::collections::{HashMap, HashSet};
use crate::currency::Currency;
use crate::game::Game;
use crate::lootbox::{LootBox, LootEntry};
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
    pub game: Game,
    money: HashMap<Currency, u32>,
    stickers: Vec<Sticker>,
    pub statistics: Statistics
}

impl Player {

    pub fn new(name: String, game: Game) -> Player {
        Player { name, game, money: HashMap::new(), stickers: Vec::new(), statistics: Statistics::default() }
    }

    pub fn add_sticker(&mut self, sticker: Sticker) {
        if !self.stickers.contains(&sticker) {
            self.stickers.push(sticker);
            self.stickers.sort();
            self.statistics.stickers_collected += 1;
        }
    }

    pub fn add_money(&mut self, currency: &Currency, amount: u32) {
        if !self.money.contains_key(currency) {
            self.money.insert((*currency).clone(), amount);
            self.game.increase_rarity();
            self.statistics.currencies_unlocked += 1;
        }
        else {
            let overflow_check = amount.overflowing_add(self.money[currency]);
            if overflow_check.1 {
                self.money.insert((*currency).clone(), u32::MAX);
                self.statistics.currencies_maxed.insert((*currency).clone());
            }
            else {
                self.money.insert((*currency).clone(), overflow_check.0);
            }
        }
    }

    pub fn can_spend_money(&self, currency: &Currency, amount: u32) -> bool {
        if !self.money.contains_key(currency) {
            false
        }
        else {
            self.money[currency] >= amount
        }
    }

    pub fn spend_money(&mut self, currency: &Currency, amount: u32) {
        if !self.can_spend_money(currency, amount) {
            panic!("Can't spend {} {}", amount, currency);
        }
        self.money.insert((*currency).clone(), self.money[currency] - amount);
    }

    pub fn can_buy_loot_box(&self, loot_box: &LootBox) -> bool {
        self.can_spend_money(&loot_box.price.0, loot_box.price.1)
    }

    pub fn buy_loot_box(&mut self, loot_box: &LootBox) {
        self.spend_money(&loot_box.price.0, loot_box.price.1);
        let mut loot = self.game.random.generate_loot_box_loot(loot_box);
        for loot_entry in loot.iter() {
            if let LootEntry::Money(currency, amount) = self {
                self.add_money(currency, *amount);
            }
            else if let LootEntry::Stickers(amount) = loot_entry {
                for _ in 0..*amount {
                    self.add_sticker(self.game.random.generate_sticker());
                }
            }
            else {
                unreachable!();
            }
        }
    }
}