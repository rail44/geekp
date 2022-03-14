// 用いるモジュールの宣言
mod item;
mod user;

// プロジェクト(crate)内からのimport
use crate::item::Item;
use crate::user::User;

fn buy(user: &mut User, cart: Vec<Item>, stocks: &mut Vec<Item>) {
    let total_price = Item::total_price(&cart);
    if !user.has_enough_money(total_price) {
        panic!("InsufficientMoney!");
    }

    for item in cart {
        let pos = stocks.iter().position(|stock| stock == &item).unwrap();
        stocks.remove(pos);

        user.owned_items.push(item);
    }
    user.wallet -= total_price;
}

fn main() {
    let name = inquire::Text::new("あなたのお名前は?").prompt().unwrap();
    let mut user = User::new(name);
    let mut stocks = Item::default_stocks();

    // 無限ループ
    loop {
        println!("{}", user);
        let cart =
            inquire::MultiSelect::new("買いたい商品を選んでください", stocks.clone()).prompt().unwrap();

        buy(&mut user, cart, &mut stocks);

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
}
