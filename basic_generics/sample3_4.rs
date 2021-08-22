/*
 * OptionやResultを使って記述する際の便利な記法
 *    cf. https://tourofrust.com/39_ja.html
 *  
 * Option/Result 内の値を取得します。列挙型が None/Err の場合、panic! します
 * 
 */
fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() -> Result<(), String> {

    // 簡潔ですが、値が存在することを仮定しており、すぐにダメになる可能性があります。
    /*
    *   my_option.unwrap() は以下のコードと同じです
    *
    *   match my_option {
    *       Some(v) => v,
    *       None => panic!("Rust によって生成されたエラーメッセージ！"),
    *   }
    *
    */
    let v = do_something_that_might_fail(42).unwrap();
    println!("発見 {}", v);

    
    /*
    *   my_result.unwrap() は以下のコードと同じです
    *
    *   match my_option {
    *       Ok(v) => v,
    *       Err(e) => panic!("Rust によって生成されたエラーメッセージ！"),
    *   }
    *
    */
    let v = do_something_that_might_fail(1).unwrap();   // パニックします
    println!("発見 {}", v);
    
    Ok(())
}

