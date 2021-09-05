/*
 * Rustでは文字列の型にstr型とString型があります。これはstr型のサンプルです
 * str型はプリミティブ型で、String型は標準ライブラリです。
 *
 * Stringは、Vecのような動的ヒープ文字列型です。文字列データを所有または変更する必要がある場合に使用します。
 * 一方でstrは不変です。メモリ内のどこかにある動的な長さのUTF-8バイトのシーケンス。サイズは不明であるため、ポインタの背後でしか処理できません。
 */


fn main(){

	// 文字列を代入すると型推論でstr型に。
	// str型は文字列スライスと呼ばれ、固定サイズで変更不可能です。
	let a = "This is ";
	let b = "a pen.";
	 
	println!("{} | {}", a, b);

	//以下はコンパイルエラーにはならないが警告が出る。
	//let mut a = "This is ";
	//warning: variable does not need to be mutable


	//str型同士で連結しようとするとコンパイルエラーになる。
	// let result = a + b;
    // エラーメッセージ
	//     error[E0369]: binary operation `+` cannot be applied to type `&str`
	//     `+` cannot be used to concatenate two `&str` strings


	// 文字列を結合したい場合にはstr変数をString型に変換するような次の3つの方法を使うようです。
	let result1 = a.to_owned() + b;
	println!("result1: {}", result1);

	let result2 = a.to_string() + b;
	println!("result2: {}", result2);

	let result3 = String::from(a) + b;
	println!("result3: {}", result3);

}
