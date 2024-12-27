use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use crate::currency::Currency;
use crate::lootbox::{LootBox, LootEntry};
use crate::sticker::Sticker;

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