/*
 *
 */
struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f はここでドロップ
}

fn main() {

    let foo = Foo { x: 42 };

    // 所有者が関数の実引数として渡されると、所有権は関数の仮引数に移動 (move) します。
    // 移動している間、所有者の値のスタックメモリは、関数呼び出しパラメータのスタックメモリにコピーされます。

    // foo の所有権は do_something に移動
    do_something(foo);

    // 移動後は、元の関数内の変数である foo は使用できなくなります
}

