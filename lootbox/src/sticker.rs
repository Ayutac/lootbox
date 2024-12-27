use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Sticker {
    pub value: u64
}

impl Display for Sticker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut arr: [char; 72] = [' '; 72]; // 72 = 64 + 8 new lines
        for i in 0..8 {
            for j in 0..8 {
                if self.value & (1 << (i * 8 + j)) != 0 {
                    arr[i * 9 + j] = '█';
                }
            }
            arr[i * 9 + 8] = '\n';
        }
        write!(f, "{}", arr.iter().collect::<String>())
    }
}

impl Sticker {

    pub fn new(value: u64) -> Self {
        Self { value }
    }

}