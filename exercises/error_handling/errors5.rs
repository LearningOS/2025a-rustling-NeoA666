// errors5.rs
//
// 此程序使用了从 errors4 修改而来的代码。
//
// 本练习涉及一些我们在课程后面才会讲到的概念，比如 `Box` 和 `From` trait。
// 现在无需深入理解它们，但如果愿意可以提前阅读。暂且把 `Box<dyn ???>` 理解为
// “我想要任何实现了 ??? 的东西”——在 Rust 对运行时安全的通常要求下，这种写法
// 显得比较宽泛！
//
// 简而言之，这种场景下使用 Box 是当你想要拥有一个值，并且只关心该值实现了某个
// 特定 trait 时。为此，Box 的类型写成 Box<dyn Trait>，Trait 就是编译器在该场景中
// 要求任意值实现的 trait。对于本练习，这个场景是可能作为 Result 返回的错误类型。
//
// 我们可以用什么来同时描述这两种错误？换句话说，是否存在一个两者都实现的 trait？
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
