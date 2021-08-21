/*
 * マルチスレッドサンプルです
 */
use std::thread;
use std::time::Duration;

fn main() {
    // スレッドを起動する
    // 引数にクロージャー(ラムダ関数)を指定
    let th = thread::spawn(|| {
        for _i in 1..10 {
            println!("{} from thread", _i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("{} from main thread. part1", i);
        thread::sleep(Duration::from_millis(3));
    }
    th.join().unwrap();

    for i in 1..5 {
        println!("{} from main thread. part2", i);
        thread::sleep(Duration::from_millis(1));
    }

}
