// quiz2.rs
//
// 这是关于以下章节的小测验：
// - 字符串（Strings）
// - 向量（Vecs）
// - 移动语义（Move semantics）
// - 模块（Modules）
// - 枚举（Enums）
//
// 让我们用函数的形式构建一个小机器。作为输入，我们将提供一个字符串和命令的列表。
// 这些命令决定了将对字符串应用什么操作。可以是：
// - 将字符串转换为大写
// - 修剪字符串（去除首尾空白）
// - 将 "bar" 附加到字符串指定的次数
// 具体形式如下：
// - 输入将是一个由长度为2的元组组成的向量，
//   第一个元素是字符串，第二个元素是命令。
// - 输出将是一个字符串向量。
//
// 这次没有提示！

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String>{
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    output.push(string.trim().to_string());
                }
                Command::Append(usize) => {
                    let mut ans = String::new();
                    for i in 0..*usize {
                        ans += &string.clone();
                    }
                    output.push(format!("{}bar", ans));
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
