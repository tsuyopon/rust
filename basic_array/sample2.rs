/*
 * 配列の宣言や表示するだけの単純なサンプル
 */
fn main() {

    // 単純な配列の宣言と表示
    let ary1 = [4, 3, 2, 1, 0];
    println!("ary1 = [{},{},{},{},{}]\r\nary1 length : {}", ary1[0], ary1[1], ary1[2], ary1[3], ary1[4], ary1.len());

    // 配列の型を指定する
    let ary2: [u8; 3] = [0x00, 0x90, 0xFF];     // let array : [型; 要素数] = [・・・]という形で宣言する
    println!("ary2[0] = 0x{:<02X}", ary2[0]);
    println!("ary2[1] = 0x{:<02X}", ary2[1]);
    println!("ary2[2] = 0x{:<02X}", ary2[2]);
    
    // 配列の中身を変更する
    let mut ary3 = [1, 2, 3, 4, 5];     // 更新するのでmutを宣言時に追加する
    println!("ary3[0] = {}", ary3[0]);
    ary3[0] = 100;                      // 更新している
    println!("ary3[0] = {}", ary3[0]);

    // 配列を初期化する
    let ary4: [u8; 3] = [0; 3];     // [初期値;要素数]
    println!("ary4[0] = {}", ary4[0]);
    println!("ary4[1] = {}", ary4[1]);
    println!("ary4[2] = {}", ary4[2]);

}
