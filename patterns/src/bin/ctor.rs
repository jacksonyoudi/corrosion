fn main() {
    let s = Second::new(10);
    println!("{}", s.value());

    let s1 = Second::default();
    println!("{}", s1.value())
}

#[derive(Default)]
pub struct Second {
    value: u64,
}

impl Second {
    pub fn new(value: u64) -> Self {
        Self {
            value
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

// impl Default for Second {
//     fn default() -> Self {
//         Self {
//             value: 0
//         }
//     }
// }



