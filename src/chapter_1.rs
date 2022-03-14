use std::fmt;

struct User {
    name: String, // 文字列
    wallet: f64, // 64bitの浮動小数点数
    owned_items: Vec<Item>, // ItemのVector(配列)
}

const DEFULT_WALLET_AMMOUNT: f64 = 500000.0;

impl User {
    // 引数リストにselfを含まない場合、クラスメソッドのような働きになる
    fn new(name: String, wallet: f64) -> User {
        User { // Userの初期化記法
            name, // `name: name` の省略記法
            wallet,
            owned_items: Vec::new(),
        }
    }  // 行末からセミコロンを除くことで、returnの省略記法になる

    // &selfを第一引数にすることでインスタンスメソッドのような働きになる
    fn _has_enough_money(&self, money: f64) -> bool {
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
                .map(|item| format!("{}", item)) // owned_itemsのそれぞれを文字列に変換して
                .collect::<Vec<_>>() // それらを要素にもったVectorとしてcollectし
                .join(", ") // ", " で結合した文字列を得る
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
    let user = User::new(name, DEFULT_WALLET_AMMOUNT);
    let stocks = Item::default_stocks();
    println!("{}", user);
    let _cart = inquire::MultiSelect::new("買いたい商品を選んでください", stocks).prompt();
    println!("{}", user);
}

// これを書くことでテストランナーの実行対象になる
// 文法的にはattributeと呼ばれるもの
#[test]
fn test_user_has_enough_money() {
    let user = User::new("hoge".to_string(), 100.0);

    assert_eq!(user._has_enough_money(100.0), true);
    assert_eq!(user._has_enough_money(80.0), true);
    assert_eq!(user._has_enough_money(105.0), false);
}
