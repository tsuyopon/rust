/*
 * str型やString型が関係するキャスト変換処理について
 */
fn main(){

	// String → &str への変換
	let str1_1: String = String::from("abc");
	let str1_2: &str = &str1_1;                        // &を付与するだけでキャストできる
	println!("{}", str1_2);  // "abc"


	// &str → String への変換
	let str2_1: &str = "abc";
	let str2_2: String = str2_1.to_string();           // to_string()を利用すると変換できる(Rust 1.19以前の記事ではto_owned()が推奨されている場合があるらしい)
	println!("{}", str2_2);  // "abc"


	// String → num への変換
	let str3: String = String::from("3");
	let num3: i32 = str3.parse().unwrap();             // .parse().unwrap()を利用すると出来ます。ちゃんと変換後の型を指定しないと動きません。
	println!("{}", num3);


	// &str → num への変換
	let str4 = "4";
	let num4: i32 = str4.parse().unwrap();
	println!("{}", num4);


	// num → String への変換
	let num5: i32 = 5;
	let str5: String = num5.to_string();               // &str → Stringと同じくto_string()を利用すると出来ます。
	println!("{}", str5);


	// num → &str への変換
	let num6: i32 = 6;
	let str6: &str = &num6.to_string();                // 一旦Stringに変換してから&を付けます。
	println!("{}", str6);


	// char → num への変換
	let c7: char = '7';
	let num7: i32 = c7 as i32 - 48;                    // asで数値に変換した後に48引きます。
	println!("{}", num7);


	// num → char への変換
	let num: i32 = 8;
	let c: char = std::char::from_digit(num as u32, 10).unwrap();  // from_digitでcharに変換します。
	println!("{}", c);



}
