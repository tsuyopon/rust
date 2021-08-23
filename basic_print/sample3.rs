/*
 * 構造体の出力
 */

fn main() {

    /*
     * デバッグ時には以下の #[derive(Debug)] を付与することでデバッグ出力ができるようになる。
     * これを付与しないとコンパイルエラーになる
     */
    #[derive(Debug)]
    struct TestStruct {
      x: f64,
      y: f64,
      side: f64,
    }
  
    let t = TestStruct{x: 2.0, y:3.0, side: 5.0};
    println!("{:?}", t);

}
