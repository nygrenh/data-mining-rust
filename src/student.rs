use std::collections::HashSet;
use std::sync::Mutex;
extern crate simple_parallel;
extern crate num_cpus;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Student<'a> {
    courses: Vec<Course<'a>>,
}

impl<'a> Student<'a> {
    pub fn new(line: &str) -> Student {
        let mut courses = HashSet::new();

        let splitted: Vec<&str> = line.split(" ").collect();
        let year = splitted[0];
        let mut i = 1;
        while i < splitted.len() {
            let month_parts: Vec<&str> = splitted[i].split('-').collect();
            courses.insert(Course {
                year: year.parse().unwrap(),
                time: splitted[i],
                code: splitted[i + 1],
                name: splitted[i + 2],
                credits: splitted[i + 3],
                grade: splitted[i + 4],
                month: month_parts[1].parse().unwrap(),
            });
            i += 5;
        }
        Student { courses: courses.into_iter().collect() }
    }

    pub fn create<'b>(data: &str) -> Box<Vec<Student>> {
        let students = Mutex::new(Box::new(Vec::new()));
        let mut pool = simple_parallel::Pool::new(num_cpus::get());
        pool.for_(data.split('\n'), |line| {
            students.lock().unwrap().push(Student::new(line));
        });
        students.into_inner().unwrap()
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Course<'a> {
    time: &'a str,
    year: i32,
    month: i8,
    code: &'a str,
    credits: &'a str,
    grade: &'a str,
    name: &'a str,
}
