/*
 * 様々な値の出力方法
 */

fn main() {

    // 配列
    println!("\n#### Array ####");
    let myarray: [i32; 3] = [1, 2, 3]; 
    println!("{:?}", myarray);   // [1, 2, 3]
    println!("{}", myarray[1]);  // [2]


    // ベクタ
    println!("\n#### Vector ####");
    let mut vect = vec![10, 20, 30];
    vect.push(40);
    println!("{:?}", vect);
    println!("{} {} {} {}", vect[0], vect[1], vect[2], vect[3]);


    // スライス
    println!("\n#### Slice ####");
    let slice = String::from("ABCDEFGH");
    let s1 = &slice[0..3];      // 0番目から3番目の手前までのスライス("ABC")
    let s2 = &slice[3..6];      // 3番目から6番目の手前までのスライス("DEF")
    println!("{} {}", s1, s2);  // => ABC DEF


    // 型の変換
    println!("\n#### Convert Type ####");
    let mytype = true;
    println!("{}", mytype as u8);


}
