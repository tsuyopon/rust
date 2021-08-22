/*
 * Result と呼ばれるジェネリックな列挙型があり、失敗する可能性のある値を返せます。
 *   cf. https://tourofrust.com/36_ja.html
 *
 * これは言語がエラーを処理する際の慣用的な方法です。
 *  enum Result<T, E> {
 *    Ok(T),
 *   Err(E),
 *  }
 * 
 */
fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() {
    let result = do_something_that_might_fail(12);

    // match は Result をエレガントに分解して、
    // すべてのケースが処理されることを保証できます！
    match result {
        Ok(v) => println!("発見 {}", v),
        Err(e) => println!("Error: {}",e),
    }
}

