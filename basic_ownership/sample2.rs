/*
 * 所有権を関数から返すこともできます。
 */
struct Foo {
    x: i32,
}

fn do_something() -> Foo {    // 所有権を関数から返す
    Foo { x: 42 }
    // 所有権は外に移動
}

fn main() {
    let foo = do_something();
    // foo は所有者になる
    // 関数のスコープの終端により、foo はドロップ
}

