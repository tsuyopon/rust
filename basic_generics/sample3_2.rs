/*
 * mainは戻り値としてResultを返すことができます。
 * sample3_1.rsからの改良版です
 * cf. https://tourofrust.com/37_ja.html
 */

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

// main は値を返しませんが、エラーを返すことがあります！
fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("発見 {}", v),
        Err(_e) => {
            // エラーをうまく処理
            
            // 何が起きたのかを説明する新しい Err を main から返します！
            return Err(String::from("main で何か問題が起きました！"));
        }
    }

    // Result の Ok の中にある unit 値によって、
    // すべてが正常であることを表現していることに注意してください。
    Ok(())
}

