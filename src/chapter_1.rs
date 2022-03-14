use std::fmt;

struct User {
    name: String,
    wallet: f64,
    owned_items: Vec<Item>,
}

const DEFULT_WALLET_AMMOUNT: f64 = 500000.0;

impl User {
    fn new(name: String) -> User {
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

// なるべくpureなrustで楽なエラーハンドリングをするためのエイリアス
// 本来はanyhowなどのライブラリを導入した方が現実の開発の上では便利になる
type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let name = inquire::Text::new("あなたのお名前は?").prompt()?;
    let user = User::new(name);
    let stocks = Item::default_stocks();
    println!("{}", user);
    let _cart = inquire::MultiSelect::new("買いたい商品を選んでください", stocks).prompt();
    println!("{}", user);
    Ok(())
}
