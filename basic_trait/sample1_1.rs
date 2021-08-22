/*
 * 構造体にimplを使って関数を紐づけることが可能です。
 * ※このサンプルにはtraitを使っていないことに注意してください。コンパイル時にWarningが表示されるだけでエラーは出ない
 */

struct Human {
    pub age: u8,
}

// 構造体Humanに実装必要な関数を定義します
impl Human {
    fn say_age(&mut self) {
        println!("{}歳です。", self.age);
    }
}

fn main() {
    let human = Human {
        age: 30,
    };
    println!("{}", human.age);
}
