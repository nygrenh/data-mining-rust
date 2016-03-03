#![feature(plugin)]
// #![plugin(clippy)]
#[macro_use] extern crate itertools;
extern crate bit_vec;

mod student;
mod appriori;

use student::Student;
use itertools::Itertools;
use std::collections::HashMap;

use bit_vec::BitVec;

fn main() {
    let data = include_str!("data-2016.csv");
    let students = Student::create(data);
    let mut courses: Vec<u32> = students.iter().flat_map(|student| &student.courses ).unique_by(|c| c.code ).map(|course| {
        course.code
    }).collect();
    courses.sort();
    let mut courses_to_ids = HashMap::new();
    let mut ids_to_courses = HashMap::new();
    for (index, course) in courses.iter().enumerate() {
        &courses_to_ids.insert(course, index);
        &ids_to_courses.insert(index, course);
    };

    let vector_students: Vec<BitVec> = students.iter().map(|student| {
        let codes: Vec<usize> = student.course_codes.iter().map(|c| *courses_to_ids.get(c).unwrap()).collect();
        let mut vector = BitVec::from_elem(courses.len(), false);

        for id in &codes {
            vector.set(*id, true);
        }

        vector
    }).collect();
    println!("Vectorized students!");
    appriori::appriori(vector_students, 0.04);
}
