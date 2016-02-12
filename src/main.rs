mod student;
mod appriori;

use student::Student;

fn main() {
    let data = include_str!("data-2016.csv");
    let students = Student::create(data);
    appriori::sequental(students, 8, 5, 0.0);
}
