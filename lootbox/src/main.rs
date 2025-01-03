mod currency;
mod random;
mod sticker;
mod player;
mod lootbox;
mod game;

use crate::currency::Currency;
use crate::game::Game;
use crate::player::Player;
use crate::random::Random;

fn main() {
    let mut currencies = vec![];
    currencies.push(Currency { name: "Copper".to_string(), rarity: 0 });
    currencies.push(Currency { name: "Silver".to_string(), rarity: 1 });
    currencies.push(Currency { name: "Gold".to_string(), rarity: 2 });
    let mut player = Player::new("Player".to_string(), Game::default());
    player.game.init();
    println!("{:?}", player);
    let mut rng = Random::from_seed(1234567890u64);
    println!("{}", rng.generate_word());
    print!("{}", rng.generate_sticker());
    let copper_box = rng.generate_loot_box(0, &currencies);
    let silver_box = rng.generate_loot_box(1, &currencies);
    let gold_box = rng.generate_loot_box(2, &currencies);
    println!("{:?}", copper_box);
    println!("{:?}", silver_box);
    println!("{:?}", gold_box);
    println!("{:?}", rng.generate_loot_box_loot(&gold_box));
}
