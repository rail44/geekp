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

    fn has_enough_money(&self, money: f64) -> bool {
        self.wallet >= money
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
            name: name.to_string(), // &str -> String
            price,
        }
    }

    fn total_price(items: &[Self]) -> f64 {
        items.iter().map(|item| item.price).sum() // itemの配列から、合計金額を計算
    }

    fn default_stocks() -> Vec<Item> {
        vec![
            Item::new("ひのきのぼう", 100.0),
            Item::new("チャージライフル", 10000.0),
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

// 名前が同じなら同じアイテムとして扱う
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// 借用元のデータを変更したい引数については&mutで宣言
fn buy(user: &mut User, cart: Vec<Item>, stocks: &mut Vec<Item>) {
    let total_price = Item::total_price(&cart);
    if !user.has_enough_money(total_price) {
        panic!("InsufficientMoney!");
    }

    for item in cart {
        let pos = stocks.iter().position(|stock| stock == &item).unwrap(); // 買うitemのstocks配列上でのindex
        stocks.remove(pos); // 在庫から削除

        user.owned_items.push(item); // 所持品へ追加
    }
    user.wallet -= total_price; // 所持金を減らす
}

fn main() {
    let name = inquire::Text::new("あなたのお名前は?").prompt().unwrap();
    // mutableなデータはlet mutで宣言
    let mut user = User::new(name);
    let mut stocks = Item::default_stocks();
    println!("{}", user);
    let cart =
        inquire::MultiSelect::new("買いたい商品を選んでください", stocks.clone()).prompt().unwrap();

    buy(&mut user, cart, &mut stocks);

    println!("{}", user);
    println!(
        "{}",
        stocks
            .iter()
            .map(|stock| format!("{}", stock))
            .collect::<Vec<_>>()
            .join(", ")
    );
}
