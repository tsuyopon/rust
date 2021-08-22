/*
 * ※ このコードのコンパイルは失敗します。
 * Human, CarはtraitのActionに指定されたrunを実装していますが、Trainはrunを実装していません。また、sample2_2.rsと異なりtrait中のActionにもrun関数が実装されていません。
 * このような場合には、Trainを定義した時点でコンパイルエラーとなります。
 */

struct Human;
struct Car;
struct Train;

// traitの宣言です。
trait Action {
    fn run(&self);
}

// Human構造体のためのActionを実装する
impl Action for Human {
    fn run(&self) {
        println!("Human running");
    }
}

// Car構造体のためのActionを実装する
impl Action for Car {
    fn run(&self) {
        println!("Car running");
    }
}

// Car構造体のためのActionを実装する
impl Action for Train {
}

fn main() {
    let human = Human;
    let car = Car;
    let train = Train;  // Trainにはrun()が実装されていないので、これはコンパイルエラーになる
    
    human.run();
    car.run();
}
