mod currency;
mod random;
mod sticker;
mod player;
mod lootbox;

use crate::currency::Currency;
use crate::player::Player;
use crate::random::Random;

fn main() {
    let mut currencies = vec![];
    currencies.push(Currency { name: "Copper".to_string(), rarity: 0 });
    currencies.push(Currency { name: "Silver".to_string(), rarity: 1 });
    currencies.push(Currency { name: "Gold".to_string(), rarity: 2 });
    let mut player = Player::new("Player".to_string());
    player.add_currency(&currencies[0], 1);
    player.add_currency(&currencies[1], 1);
    player.add_currency(&currencies[2], 1);
    println!("{:?}", player);
    let mut rng = Random::from_seed(1234567890u64);
    println!("{}", rng.generate_word());
    print!("{}", rng.generate_sticker().display());
}
