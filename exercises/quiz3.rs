// 为ReportCard添加泛型，支持任意可格式化的成绩类型
pub struct ReportCard<G> {
    pub grade: G,
    pub student_name: String,
    pub student_age: u8,
}

impl<G: std::fmt::Display> ReportCard<G> {
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
        // 泛型参数G为f32类型
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
        // 泛型参数G为&str类型（也可使用String）
        let report_card = ReportCard {
            grade: "A+", // 修改为字母成绩
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
