use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use crate::sticker::Sticker;

pub struct Rng {
    pub rng: StdRng
}

impl Default for Rng {
    fn default() -> Self {
        Rng { rng: StdRng::from_os_rng()}
    }
}

impl Rng {

    pub fn from_seed(seed: u64) -> Self {
        Rng { rng: StdRng::seed_from_u64(seed) }
    }

    pub fn generate_word(&mut self) -> String {
        String::from("Word") + self.rng.next_u32().to_string().as_str()
    }

    pub fn generate_sticker(&mut self) -> Sticker {
        Sticker::new(self.rng.next_u64())
    }

}