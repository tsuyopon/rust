/*
 * traitを使ったサンプルです。
 * trait は、ある型が実装しなければならない機能をコンパイラに伝える機能を持ちます
 *
 * この例ではHumanとCarはtraitであるActionに指定されたrun関数を実装しなければなりません。
 */

struct Human;
struct Car;

// traitの宣言です。このtraitはrun関数を持たなければなりません。
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

fn main() {
    let human = Human;
    let car = Car;
    
    human.run();
    car.run();
}
