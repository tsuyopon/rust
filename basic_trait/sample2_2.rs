/*
 * この例では、Human, Car, TrainはtraitのActionによりrunを実装することが強制されている。
 * HumanとCarでは独自でrun関数を実装しているが、Trainは独自実装していないので、trait Actionに定義されているrun関数が呼び出される
 */

struct Human;
struct Car;
struct Train;

// traitの宣言です。
trait Action {
    fn run(&self) {
        println!("running");
    }
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
    let train = Train;
    
    human.run();
    car.run();
    train.run();    // Trainの実装ではrunは定義されていないので、trait中に定義されたデフォルトが利用される
}
