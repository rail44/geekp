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

fn buy(user: &mut User, cart: Vec<Item>, stocks: &mut Vec<Item>) -> Result<(), Error> {
    let total_price = Item::total_price(&cart);
    if !user.has_enough_money(total_price) {
        return Err(GeekpError::InsufficientMoney.into());
    }

    for item in cart {
        let pos = stocks.iter().position(|stock| stock == &item).unwrap(); // 買うitemのstocks配列上でのindex
        stocks.remove(pos); // 在庫から削除

        user.owned_items.push(item); // 所持品へ追加
    }
    user.wallet -= total_price; // 所持金を減らす

    Ok(())
}

fn main() {
    let name = inquire::Text::new("あなたのお名前は?").prompt().unwrap();
    let mut user = User::new(name);
    let mut stocks = Item::default_stocks();
    println!("{}", user);
    let cart =
        inquire::MultiSelect::new("買いたい商品を選んでください", stocks.clone()).prompt().unwrap();

    buy(&mut user, cart, &mut stocks).unwrap();

    println!(
        "{}",
        stocks
            .iter()
            .map(|stock| format!("{}", stock))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("{}", user);
}
