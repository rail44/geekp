mod item;
mod user;

use crate::item::Item;
use crate::user::User;
use std::fmt;

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
    loop {
        println!("{}", user);
        let cart =
            inquire::MultiSelect::new("買いたい商品を選んでください", stocks.clone()).prompt()?;

        buy(&mut user, cart, &mut stocks)?;

        println!(
            "{}",
            stocks
                .iter()
                .map(|stock| format!("{stock}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
