use std::fmt;

struct User {
    name: String,
    wallet: f64,
    owned_items: Vec<Item>,
}

const DEFULT_WALLET_AMMOUNT: f64 = 250000.0;

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
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

#[derive(Clone)]
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

    fn total_price(items: &[Self]) -> f64 {
        items.iter().map(|item| item.price).sum()
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

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// なるべくpureなrustで楽なエラーハンドリングをするためのエイリアス
// 本来はanyhowなどのライブラリを導入した方が現実の開発の上では便利になる
type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
enum GeekpError {
    InsufficientMoney,
}

impl fmt::Display for GeekpError {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        w.write_fmt(format_args!("{:?}", self))
    }
}
impl std::error::Error for GeekpError {}

fn buy(user: &mut User, items: Vec<Item>, stocks: &mut Vec<Item>) -> Result<(), Error> {
    let total_price = Item::total_price(&items);
    if total_price > user.wallet {
        return Err(GeekpError::InsufficientMoney.into());
    }

    for item in items {
        stocks.retain(|stock| stock != &item);
        user.owned_items.push(item);
    }
    user.wallet -= total_price;

    Ok(())
}

fn main() -> Result<(), Error> {
    let name = inquire::Text::new("あなたのお名前は?").prompt()?;
    let mut user = User::new(name);
    let mut stocks = Item::default_stocks();
    println!("{}", user);
    let cart =
        inquire::MultiSelect::new("買いたい商品を選んでください", stocks.clone()).prompt()?;

    buy(&mut user, cart, &mut stocks)?;

    println!("{}", user);
    println!(
        "{}",
        stocks
            .iter()
            .map(|stock| format!("{stock}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    Ok(())
}
