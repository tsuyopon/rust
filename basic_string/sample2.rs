/*
 * Rustでは文字列の型にstr型とString型があります。これはString型のサンプルです
 * str型はプリミティブ型で、String型は標準ライブラリです。
 *
 * Stringは、Vecのような動的ヒープ文字列型です。文字列データを所有または変更する必要がある場合に使用します。
 * 一方でstrは不変です。メモリ内のどこかにある動的な長さのUTF-8バイトのシーケンス。サイズは不明であるため、ポインタの背後でしか処理できません。
 */


fn main(){

	/*
	 * String型は、str型と異なり可変長文字列を扱えて変更も可能
	 */
	//新しい空のStringを生成
	let a1 = String::new();
	println!("a1: {}", a1);

	//文字列を代入しながら初期化
	let a2 = String::from("This is a ");
	println!("a2: {}", a2);

	//文字列リテラルにto_string()でも可
	let a3 = "This is a ".to_string();
	println!("a3: {}", a3);


	// String型同士は連結できない
	let str1 = String::from("This is a ");
	println!("str1: {}", str1);
	let str2 = "a pen.".to_string();
	println!("str2: {}", str2);

	//以下でコンパイルエラー
	//let result = a + b;
	//error[E0308]: mismatched types
	//expected &str, found struct `std::string::String`
	//help: consider borrowing here: `&b`

	// String同士を連結したい場合には後ろの変数の頭に&を入れて借用形にする。
	let str1_2 = str1 + &str2;
	println!("str1 + str2: {}", str1_2);

}
