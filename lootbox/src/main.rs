mod currency;
mod rng;
mod sticker;
mod player;

use rand::SeedableRng;
use crate::currency::Currency;
use crate::player::Player;
use crate::rng::Rng;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Goody {
    pub name: String,
    pub rarity: u8
}



fn main() {
    let mut currencies = vec![];
    currencies.push(Currency { name: "Copper".to_string(), rarity: 0 });
    currencies.push(Currency { name: "Silver".to_string(), rarity: 1 });
    currencies.push(Currency { name: "Gold".to_string(), rarity: 2 });
    let mut player = Player::new("Player".to_string());
    player.currencies.insert(currencies[0].clone(), 1);
    player.currencies.insert(currencies[1].clone(), 1);
    player.currencies.insert(currencies[2].clone(), 1);
    println!("{:?}", player);
    let mut rng = Rng::from_seed(1234567890u64);
    println!("{}", rng.generate_word());
    println!("{}", rng.generate_sticker().display());
}
