use std::fmt::Display;

// Make ReportCard generic over any grade type `T`
struct ReportCard<T> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// Implement print for any type `T` that implements Display
impl<T: Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    let numeric_report = ReportCard {
        grade: 2.1,
        student_name: "Tom Wriggle".to_string(),
        student_age: 12,
    };
    println!("{}", numeric_report.print());

    let alphabetic_report = ReportCard {
        grade: "A+",
        student_name: "Gary Plotter".to_string(),
        student_age: 11,
    };
    println!("{}", alphabetic_report.print());
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
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
