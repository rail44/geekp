use std::fmt;

#[derive(Clone)]
pub struct Item {
    name: String,
    price: f64,
}

impl Item {
    pub fn new(name: &str, price: f64) -> Item {
        Item {
            name: name.to_string(),
            price,
        }
    }

    pub fn total_price(items: &[Self]) -> f64 {
        items.iter().map(|item| item.price).sum()
    }

    pub fn default_stocks() -> Vec<Item> {
        vec![
            Item::new("MacBook Pro", 239800.0),
            Item::new("HHKB Pro", 25300.0),
            Item::new("SICP", 5060.0),
        ]
    }
}

impl fmt::Display for Item {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        w.write_fmt(format_args!("{}[{}円]", self.name, self.price))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
