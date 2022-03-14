use std::fmt;

struct User {
    name: String, // 文字列
    wallet: f64, // 64bitの浮動小数点数
    owned_items: Vec<Item>, // ItemのVector(配列)
}

const DEFULT_WALLET_AMMOUNT: f64 = 500000.0;

impl User {
    fn new(name: String) -> User {
        User {
            name, // `name: name` の省略記法
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
                .map(|item| format!("{}", item))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

struct Item {
    name: String,
    price: f64,
}

impl Item {
    fn new(name: &str, price: f64) -> Item {
        Item {
            name: name.to_string(),
            price,
        }
    }

    fn default_stocks() -> Vec<Item> {
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

fn main() {
    let name = inquire::Text::new("あなたのお名前は?").prompt().unwrap();
    let user = User::new(name);
    let stocks = Item::default_stocks();
    println!("{}", user);
    let _cart = inquire::MultiSelect::new("買いたい商品を選んでください", stocks).prompt();
    println!("{}", user);
}
