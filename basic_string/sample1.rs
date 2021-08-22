

fn main() {

	// 文字列を初期化する
	let mut name = String::from("Yamada");

	// 別の文字列を設定する
	name = "Tanaka".to_string();

	// 文字列に追加する
	name.push_str(" Taro");

	println!("{}", name)

}
