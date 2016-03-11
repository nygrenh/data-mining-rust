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
    println!("Number of courses {:?}", courses.len());
    courses = courses.into_iter().filter(|c| supported(c, &students, 0.04) ).collect();
    println!("Number of courses {:?}", courses.len());
    for (index, course) in courses.iter().enumerate() {
        &courses_to_ids.insert(course, index);
        &ids_to_courses.insert(index, course);
    }

    let vector_students: Vec<BitSet> =
        students.iter()
                .map(|student| {
                    let codes: Vec<usize> = student.course_codes
                                                   .iter()
                                                   .filter(|c| courses_to_ids.contains_key(c))
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

fn supported(course_code: &u32, students: &[Student], desired_support: f32) -> bool {
    let mut count = 0;
    for student in students {
        if student.course_codes.contains(&course_code) {
            count += 1;
        }
    }
    (count as f32 / students.len() as f32) as f32 >= desired_support
}
