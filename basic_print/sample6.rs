/*
 * 様々な型を出力するサンプル  (標準搭載されているgdb!マクロとほぼ同じ)
 */
macro_rules! printv {
    ( $($x:expr),* ) => {{
        $(print!("{}={:?} ", stringify!($x), $x);)*
        println!();
    }}
}

fn main() { 
	let a = 10;
	let b = Some(2);
	let c = vec!["hello", "world"];
	printv!(a, b, c);
}
