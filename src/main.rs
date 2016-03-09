#![feature(plugin)]
// #![plugin(clippy)]
#[macro_use]
extern crate itertools;
extern crate bit_set;

mod student;
mod appriori;

use student::Student;
use itertools::Itertools;
use std::collections::HashMap;

use bit_set::BitSet;

fn main() {
    let data = include_str!("data-2016.csv");
    let students = Student::create(data);
    let mut courses: Vec<u32> = students.iter()
                                        .flat_map(|student| &student.courses)
                                        .unique_by(|c| c.code)
                                        .map(|course| course.code)
                                        .collect();
    courses.sort();
    let mut courses_to_ids = HashMap::new();
    let mut ids_to_courses = HashMap::new();
    for (index, course) in courses.iter().enumerate() {
        &courses_to_ids.insert(course, index);
        &ids_to_courses.insert(index, course);
    }

    let vector_students: Vec<BitSet> =
        students.iter()
                .map(|student| {
                    let codes: Vec<usize> = student.course_codes
                                                   .iter()
                                                   .map(|c| *courses_to_ids.get(c).unwrap())
                                                   .collect();
                    let mut vector = BitSet::with_capacity(courses.len());

                    for code in codes {
                        // let real = &ids_to_courses.get(&code.clone()).unwrap();
                        // println!("Inserting {:?} into codes... It should be: {:?}", code, real);
                        vector.insert(code);
                    }
                    vector
                })
                .collect();

    appriori::appriori(vector_students, 0.04, courses.len());
}
