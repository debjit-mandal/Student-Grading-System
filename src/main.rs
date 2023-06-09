use std::collections::HashMap;
use std::io;
use std::iter::Sum;

// Structure to represent a student
struct Student {
    name: String,
    grades: HashMap<String, f32>,
}

impl Student {
    // Calculate the average grade for a student
    fn calculate_average_grade(&self) -> f32 {
        let total: f32 = self.grades.values().sum();
        let count = self.grades.len() as f32;
        total / count
    }
}

// Structure to represent a subject
struct Subject {
    name: String,
}

// Structure to represent the grading system
struct GradingSystem {
    students: HashMap<String, Student>,
    subjects: HashMap<String, Subject>,
}

impl GradingSystem {
    // Register a new student
    fn register_student(&mut self, name: String) {
        let student = Student {
            name: name.clone(),
            grades: HashMap::new(),
        };
        self.students.insert(name, student);
    }

    // Add a new subject
    fn add_subject(&mut self, name: String) {
        let subject = Subject { name: name.clone() };
        self.subjects.insert(name, subject);
    }

    // Add a grade for a student and subject
    fn add_grade(&mut self, student_name: &str, subject_name: &str, grade: f32) {
        if let Some(student) = self.students.get_mut(student_name) {
            student.grades.insert(subject_name.to_string(), grade);
        }
    }

    // Generate a grade report for a student
    fn generate_grade_report(&self, student_name: &str) -> Option<String> {
        if let Some(student) = self.students.get(student_name) {
            let mut report = format!("Grade Report for {}\n", student.name);
            for (subject_name, grade) in &student.grades {
                report.push_str(&format!("Subject: {}, Grade: {}\n", subject_name, grade));
            }
            Some(report)
        } else {
            None
        }
    }

    // Calculate the average grade for a subject
    fn calculate_subject_average_grade(&self, subject_name: &str) -> Option<f32> {
        let mut total = 0.0;
        let mut count = 0;
        for student in self.students.values() {
            if let Some(grade) = student.grades.get(subject_name) {
                total += *grade;
                count += 1;
            }
        }
        if count > 0 {
            Some(total / count as f32)
        } else {
            None
        }
    }

    // Calculate the average grade for all subjects
    fn calculate_overall_average_grade(&self) -> Option<f32> {
        let mut total = 0.0;
        let mut count = 0;
        for student in self.students.values() {
            total += student.calculate_average_grade();
            count += 1;
        }
        if count > 0 {
            Some(total / count as f32)
        } else {
            None
        }
    }

    // Sort and display student records
    fn display_student_records(&self) {
        let mut students: Vec<_> = self.students.values().collect();
        students.sort_by(|a, b| a.name.cmp(&b.name));

        println!("Student Records:");
        for student in students {
            println!("Student: {}", student.name);
            for (subject_name, grade) in &student.grades {
                println!("Subject: {}, Grade: {}", subject_name, grade);
            }
            println!("Average Grade: {}", student.calculate_average_grade());
            println!("-------------------------");
        }
    }

    // Sort and display subject grades
    fn display_subject_grades(&self) {
        let mut subjects: Vec<_> = self.subjects.values().collect();
        subjects.sort_by(|a, b| a.name.cmp(&b.name));

        println!("Subject Grades:");
        for subject in subjects {
            println!("Subject: {}", subject.name);
            for student in self.students.values() {
                if let Some(grade) = student.grades.get(&subject.name) {
                    println!("Student: {}, Grade: {}", student.name, grade);
                }
            }
            println!("-------------------------");
        }
    }

    // Perform statistical analysis on subject grades
    fn perform_statistical_analysis(&self, subject_name: &str) {
        let mut grades: Vec<_> = self
            .students
            .values()
            .filter_map(|student| student.grades.get(subject_name))
            .collect();
        grades.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let count = grades.len();
        let average = grades.iter().map(|&x| *x).sum::<f32>() / count as f32;
        let median = if count % 2 == 0 {
            (grades[count / 2 - 1] + grades[count / 2]) / 2.0
        } else {
            *grades[count / 2]
        };
        let min_grade = *grades.first().unwrap();
        let max_grade = *grades.last().unwrap();

        println!("Statistical Analysis for Subject '{}'", subject_name);
        println!("Count: {}", count);
        println!("Average: {:.2}", average);
        println!("Median: {:.2}", median);
        println!("Minimum Grade: {:.2}", min_grade);
        println!("Maximum Grade: {:.2}", max_grade);
    }
}

fn main() {
    let mut grading_system = GradingSystem {
        students: HashMap::new(),
        subjects: HashMap::new(),
    };
    loop {
        println!("1. Register a student");
        println!("2. Add a subject");
        println!("3. Add a grade");
        println!("4. Generate a grade report");
        println!("5. Calculate average grade for a subject");
        println!("6. Calculate overall average grade");
        println!("7. Display student records");
        println!("8. Display subject grades");
        println!("9. Perform statistical analysis on subject grades");
        println!("10. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter student name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read input");
                grading_system.register_student(name.trim().to_string());
            }
            2 => {
                println!("Enter subject name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read input");
                grading_system.add_subject(name.trim().to_string());
            }
            3 => {
                println!("Enter student name:");
                let mut student_name = String::new();
                io::stdin()
                    .read_line(&mut student_name)
                    .expect("Failed to read input");
                println!("Enter subject name:");
                let mut subject_name = String::new();
                io::stdin()
                    .read_line(&mut subject_name)
                    .expect("Failed to read input");
                println!("Enter grade:");
                let mut grade = String::new();
                io::stdin().read_line(&mut grade).expect("Failed to read input");
                let grade: f32 = match grade.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid grade!");
                        continue;
                    }
                };
                grading_system.add_grade(
                    student_name.trim(),
                    subject_name.trim(),
                    grade,
                );
            }
            4 => {
                println!("Enter student name:");
                let mut student_name = String::new();
                io::stdin()
                    .read_line(&mut student_name)
                    .expect("Failed to read input");
                if let Some(report) = grading_system.generate_grade_report(student_name.trim()) {
                    println!("{}", report);
                } else {
                    println!("Student not found!");
                }
            }
            5 => {
                println!("Enter subject name:");
                let mut subject_name = String::new();
                io::stdin()
                    .read_line(&mut subject_name)
                    .expect("Failed to read input");
                if let Some(average) = grading_system.calculate_subject_average_grade(subject_name.trim()) {
                    println!("Average grade for subject '{}': {:.2}", subject_name.trim(), average);
                } else {
                    println!("No grades available for subject '{}'", subject_name.trim());
                }
            }
            6 => {
                if let Some(average) = grading_system.calculate_overall_average_grade() {
                    println!("Overall average grade: {:.2}", average);
                } else {
                    println!("No grades available");
                }
            }
            7 => grading_system.display_student_records(),
            8 => grading_system.display_subject_grades(),
            9 => {
                println!("Enter subject name:");
                let mut subject_name = String::new();
                io::stdin()
                    .read_line(&mut subject_name)
                    .expect("Failed to read input");
                grading_system.perform_statistical_analysis(subject_name.trim());
            }
            10 => break,
            _ => println!("Invalid choice!"),
        }
        println!();
    }
}
