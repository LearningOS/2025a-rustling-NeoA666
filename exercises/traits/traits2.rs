// traits2.rs
//
// 你的任务是为字符串向量实现 trait `AppendBar`。
// 实现此 trait 时，请考虑一下对字符串向量“追加 \"Bar\"”意味着什么。
//
// 这次没有样板代码，你可以直接实现！
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>
{
    fn append_bar(mut self) -> Self{
        self.push("Bar".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
