/*
 * 標準で用意されている dbg! マクロを利用して変数のデバッグを出力する
 */
fn main() {
	let a = 10;
	let b = Some(2);
	let c = vec!["hello", "world"];
	dbg!(a, b, c);
}
