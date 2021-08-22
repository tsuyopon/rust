/*
 * 参照外し
 * &mut による参照では、* 演算子によって参照を外す (dereference) ことで、所有者の値を設定できます。
 *   cf. https://tourofrust.com/50_ja.html
 */
fn main() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // 所有者の値を取得
    *f = 13;      // 参照の所有者の値を設定
    println!("{}", bar);
    println!("{}", foo);
}
