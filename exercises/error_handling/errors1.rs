// errors1.rs
//
// 如果你传入一个空字符串，这个函数会拒绝生成要打印在名牌上的文本。
// 如果它能解释一下问题所在，而不是有时候只返回 `None`，会更好。
// 幸运的是，Rust 有一个类似于 `Result` 的结构体，可以用来表达错误情况。让我们用它吧！
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.


pub fn generate_nametag_text(name: String) -> Result<String,String>{
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
