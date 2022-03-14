use std::fmt;

fn is_discount_available() -> bool {
    true
}

#[derive(Clone)]
pub enum Category {
    Weapon,
    Laptop,
    Keyboard,
    Book,
}

#[derive(Clone)]
pub struct Item {
    name: String,
    price: f64,
    category: Category,
}

impl Item {
    fn new(name: &str, price: f64, category: Category) -> Item {
        Item {
            name: name.to_string(),
            price,
            category,
        }
    }

    pub fn total_price(items: &[Self]) -> f64 {
        items.iter().map(|item| item.discounted_price()).sum()
    }

    pub fn default_stocks() -> Vec<Item> {
        vec![
            Item::new("ひのきのぼう", 100.0, Category::Weapon),
            Item::new("チャージライフル", 10000.0, Category::Weapon),
            Item::new("MacBook Pro", 239800.0, Category::Laptop),
            Item::new("HHKB Pro", 25300.0, Category::Keyboard),
            Item::new("SICP", 5060.0, Category::Book),
            Item::new("Engineers in VOYAGE ― 事業をエンジニアリングする技術者たち", 1980.0, Category::Book),
        ]
    }

    fn discounted_price(&self) -> f64 {
        if !is_discount_available() {
            return self.price;
        }

        match &self.category {
            Category::Book => self.price * 0.9,
            Category::Weapon => self.price * 0.8,
            _ => self.price,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        w.write_fmt(format_args!("{}[{}円]", self.name, self.discounted_price()))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
