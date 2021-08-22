/*
 * ドロップは階層的
 *    cf. https://tourofrust.com/45_ja.html
 */
struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // foo が最初にドロップ
    // 次に foo.bar がドロップ
}

