#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Sticker {
    pub value: u64
}

impl Sticker {

    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn display(&self) -> String {
        let mut arr: [char; 72] = [' '; 72]; // 72 = 64 + 8 new lines
        for i in 0..8 {
            for j in 0..8 {
                if self.value & (1 << (i * 8 + j)) != 0 {
                    arr[i * 9 + j] = '█';
                }
            }
            arr[i * 9 + 8] = '\n';
        }
        arr.iter().collect::<String>()
    }
}