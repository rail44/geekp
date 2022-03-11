use crate::item::Item;
use std::fmt;

pub struct User {
    name: String,
    pub wallet: f64,
    pub owned_items: Vec<Item>,
}

const DEFULT_WALLET_AMMOUNT: f64 = 250000.0;

impl User {
    pub fn new(name: String) -> User {
        User {
            name,
            wallet: DEFULT_WALLET_AMMOUNT,
            owned_items: Vec::new(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        w.write_fmt(format_args!(
            "名前: {}, 所持金: {}円, 所持アイテム: {}",
            self.name,
            self.wallet,
            self.owned_items
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}
