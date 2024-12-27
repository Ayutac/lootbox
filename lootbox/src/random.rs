use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use crate::currency::Currency;
use crate::lootbox::{LootBox, LootEntry, LootTable};
use crate::lootbox::LootEntry::{Money, Stickers};
use crate::sticker::Sticker;

#[derive(Clone, Debug)]
pub struct Random {
    pub rng: StdRng
}

impl Default for Random {
    fn default() -> Self {
        Random { rng: StdRng::from_os_rng()}
    }
}

impl Random {

    pub fn from_seed(seed: u64) -> Self {
        Random { rng: StdRng::seed_from_u64(seed) }
    }

    pub fn generate_word(&mut self) -> String {
        String::from("Word") + self.rng.next_u32().to_string().as_str()
    }

    pub fn generate_sticker(&mut self) -> Sticker {
        Sticker::new(self.rng.next_u64())
    }

    pub fn generate_currency(&mut self, rarity: u8) -> Currency {
        Currency {name: self.generate_word(), rarity}
    }

    pub fn generate_loot_box(&mut self, rarity: u8, currencies: &Vec<Currency>) -> LootBox {
        if currencies.is_empty() {
            panic!("Currencies cannot be empty!");
        }
        let mut loot_table: Vec<(LootEntry, u16)> = Vec::new();
        // get some specific currencies
        let mut next_rarity_currency = None;
        let mut same_rarity_currency = None;
        let mut previous_rarity_currency = None;
        let mut second_previous_rarity_currency = None;
        for currency in currencies {
            if currency.rarity == rarity + 1 {
                next_rarity_currency = Some(currency.clone());
            }
            else if currency.rarity == rarity {
                same_rarity_currency = Some(currency.clone());
            }
            else if currency.rarity + 1 == rarity {
                previous_rarity_currency = Some(currency.clone());
            }
            else if currency.rarity + 2 == rarity {
                second_previous_rarity_currency = Some(currency.clone());
            }
        }
        let mut price_currency = same_rarity_currency.clone();
        // fill the loot table with currencies
        if next_rarity_currency.is_some() {
            loot_table.push((
                Money(next_rarity_currency.unwrap(), self.rng.random_range(1u32..16u32)), // reward
                self.rng.random_range(1u16..6u16) // weight
            ));
        }
        if same_rarity_currency.is_some() {
            loot_table.push((
                Money(same_rarity_currency.unwrap(), self.rng.random_range(5u32..31u32)), // reward
                self.rng.random_range(3u16..21u16) // weight
            ));
        }
        if previous_rarity_currency.is_some() {
            loot_table.push((
                Money(previous_rarity_currency.clone().unwrap(), self.rng.random_range(20u32..150u32)), // reward
                self.rng.random_range(5u16..21u16) // weight
            ));
            loot_table.push((
                Money(previous_rarity_currency.unwrap(), self.rng.random_range(20u32..150u32)), // reward
                self.rng.random_range(5u16..21u16) // weight
            ));
        }
        if second_previous_rarity_currency.is_some() {
            loot_table.push((
                Money(second_previous_rarity_currency.clone().unwrap(), self.rng.random_range(100u32..501u32)), // reward
                self.rng.random_range(5u16..21u16) // weight
            ));
            loot_table.push((
                Money(second_previous_rarity_currency.clone().unwrap(), self.rng.random_range(100u32..501u32)), // reward
                self.rng.random_range(5u16..21u16) // weight
            ));
            loot_table.push((
                Money(second_previous_rarity_currency.unwrap(), self.rng.random_range(100u32..501u32)), // reward
                self.rng.random_range(5u16..21u16) // weight
            ));
        }
        // add stickers to the loot table
        loot_table.push((
            Stickers(self.rng.random_range(1u32..(rarity+2) as u32)),
            self.rng.random_range(10u16..51u16)
        ));
        // get price currency;
        if price_currency.is_none() {
            price_currency = Some(currencies[0].clone());
        }
        LootBox {
            name: self.generate_word(),
            rarity,
            price: (price_currency.unwrap(), self.rng.random_range(5u32..41u32)),
            no_of_rewards: self.rng.random_range(1u8..6u8),
            loot_table: LootTable::new(loot_table)
        }
    }

    pub fn generate_loot_box_loot(&mut self, loot_box: &LootBox) -> Vec<LootEntry> {
        let mut loot: Vec<LootEntry> = Vec::with_capacity(loot_box.no_of_rewards as usize);
        let size = loot_box.loot_table.size();
        for _ in 0..loot_box.no_of_rewards {
            let selected = self.rng.random_range(0u32..size);
            loot.push(loot_box.loot_table.select(selected).unwrap())
        }
        loot
    }

}