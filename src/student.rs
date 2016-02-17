use std::collections::HashSet;
use std::sync::Mutex;
use std::fmt;

extern crate simple_parallel;
extern crate num_cpus;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Student<'a> {
    pub courses: Vec<Course<'a>>,
    pub course_codes: Vec<u32>
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
                code: splitted[i + 1].parse().unwrap(),
                name: splitted[i + 2],
                credits: splitted[i + 3],
                grade: splitted[i + 4],
                month: month_parts[1].parse().unwrap(),
            });
            i += 5;
        }
        let courses2: Vec<Course> = courses.into_iter().collect();
        let codes = Student::collect_course_codes(&courses2);
        Student { courses: courses2, course_codes: codes }
    }

    pub fn create(data: &str) -> Vec<Student> {
        let students = Mutex::new(Vec::new());
        let mut pool = simple_parallel::Pool::new(num_cpus::get());
        pool.for_(data.split('\n'), |line| {
            students.lock().unwrap().push(Student::new(line));
        });
        students.into_inner().unwrap()
    }

    fn collect_course_codes(courses: &[Course]) -> Vec<u32> {
        let mut res = Vec::new();
        for course in courses {
            res.push(course.code);
        }
        res
    }
}

#[derive(Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Course<'a> {
    pub code: u32,
    pub name: &'a str,
    pub time: &'a str,
    pub year: u16,
    pub month: u8,
    pub credits: &'a str,
    pub grade: &'a str,
}

impl<'a> fmt::Debug for Course<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Course#{}", self.code)
    }
}

impl<'a> PartialEq for Course<'a> {
    fn eq(&self, other: &Course) -> bool {
        self.code == other.code
    }
}
