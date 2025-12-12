// quiz3.rs
//
// 本测验考察：
// - 泛型
// - 特质（traits）
//
// 一个虚构的魔法学校有一个用 Rust 编写的新成绩单生成系统！目前该系统仅支持生成以数字表示成绩的成绩单（例如 1.0 -> 5.5）。但学校也会发放字母等级（A+ -> F-），需要能够打印这两种类型的成绩单！
//
// 在 struct ReportCard 和 impl 块中进行必要的代码更改以支持字母等级成绩单。在第二个测试中将 Grade 改为 "A+"，以表明你的更改已支持字母等级。
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
