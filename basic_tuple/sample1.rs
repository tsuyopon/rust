/*
 * タプルを使った簡単なサンプルプログラムです
 */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(1, 2, 3);
    let Point (x,y,z) = Point(10, 20, 30);
    
    // case1: インデックスを指定することが可能
    println!("{} {} {}", black.0, black.1, black.2);

    // case2: 要素を指定することもできる
    println!("{} {} {}", x, y, z);
}
