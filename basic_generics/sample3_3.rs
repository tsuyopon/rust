/*
 * mainは戻り値としてResultを返すことができます。そのための強力な演算子として?が用意されています。
 * sample3_2.rsからの改良版です
 * cf. https://tourofrust.com/38_ja.html
 */

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() -> Result<(), String> {

    /*
     *   do_something_that_might_fail()?は以下のコードと同じです。
     * 
     *   match do_something_that_might_fail() {
     *       Ok(v) => v,
     *       Err(e) => return Err(e),
     *   }
     */
    let v = do_something_that_might_fail(42)?;     // 強力な演算子"?"

    println!("発見 {}", v);
    Ok(())
}
