use crate::currency::Currency;
use crate::lootbox::LootBox;
use crate::random::Random;

#[derive(Clone, Debug)]
pub struct Game {
    pub random: Random,
    currencies: Vec<Currency>,
    loot_boxes: Vec<LootBox>,
}

impl Default for Game {
    fn default() -> Self {
        Game::new(Random::default(), None)
    }
}

impl Game {
    pub fn new(random: Random, start_currencies: Option<Vec<Currency>>) -> Self {
        if start_currencies.is_none() {
            Game { random, currencies: Vec::new(), loot_boxes: Vec::new() }
        }
        else {
            Game { random, currencies: start_currencies.unwrap(), loot_boxes: Vec::new() }
        }
    }

    pub fn highest_rarity(&self) -> Option<u8> {
        if self.currencies.is_empty() {
            None
        }
        else {
            let mut highest_rarity = 0;
            for currency in &self.currencies {
                if highest_rarity < currency.rarity {
                    highest_rarity = currency.rarity;
                }
            }
            Some(highest_rarity)
        }
    }

    pub fn init(&mut self) {
        if self.currencies.is_empty() {
            self.currencies.push(self.random.generate_currency(0u8));
            self.currencies.push(self.random.generate_currency(1u8));
        }
        for i in 0..self.highest_rarity().unwrap() {
            self.loot_boxes.push(self.random.generate_loot_box(i, &self.currencies));
        }
    }

    pub fn increase_rarity(&mut self) {
        let highest_rarity_opt = self.highest_rarity();
        if highest_rarity_opt.is_none() {
            panic!("Games hasn't been initialized yet!");
        }
        let highest_rarity = highest_rarity_opt.unwrap();
        self.currencies.push(self.random.generate_currency(highest_rarity+1));
        self.loot_boxes.push(self.random.generate_loot_box(highest_rarity, &self.currencies));
    }

}