/*
 * 構造体にimplを使って関数を紐づけることが可能です。
 * このサンプルはtraitを使っていて、「impl Action for Human」によってHumanは必ずsay_ageを実装することが義務付けられるようになっています。
 */

struct Human {
    pub age: u8,
}

trait Action {
    fn say_age(&mut self);
}

// 構造体Humanに実装必要な関数を定義します
impl Action for Human {

    // say_age関数をコメントアウトすると、trait Actionでsay_ageの実装の強制が満たされなくなり、コンパイル時にエラーとなります。
    fn say_age(&mut self) {
        println!("{}歳です。", self.age);
    }
}

fn main() {
    let mut human = Human {
        age: 30,
    };
    println!("{}", human.age);
}
