/*
 * クロージャーのサンプルです。
 * クロージャー は他の言語で言うところの無名関数やラムダ式に似ています。
 */
fn main() {

	// 二乗する関数
	let square = | x: i32 | {
		x * x
	};
	println!("{}", square(9));


	let msg = String::from("Hello");    // クロージャー外変数msg
	let func = move || {                // 所有権をクロージャーに移動すること宣言
		println!("{}", msg);            // 参照したクロージャー外変数の所有権はクロージャーに移動
	};                                  // クロージャー終了時に所有者が不在となり解放される
	func();                             // クロージャーを呼び出す
// println!("{}", msg);		// 解放済領域を参照しようとするのでエラーとなる
}
