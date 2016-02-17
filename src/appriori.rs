use itertools::Itertools;
use std::sync::Mutex;

extern crate simple_parallel;
extern crate num_cpus;

use student::Student;
use student::Course;

pub fn appriori(students: Vec<Student>,
                 desired_support: f32) {
    println!("Starting appriori with support {:?}", desired_support);
    println!("Generating level 1...");
    let mut courses: Vec<Vec<&Course>> = students.iter().flat_map(|student| &student.courses ).unique_by(|c| c.code ).map(|course| {
        let mut inner: Vec<&Course> = Vec::new();
        inner.push(course);
        inner
    }).collect();
    courses.sort();

    println!("First level has been generated!");
    let mut level = 1;
    while !courses.is_empty() {
        let safe_next_level: Mutex<Vec<Vec<&Course>>> = Mutex::new(Vec::new());
        let mut pool = simple_parallel::Pool::new(num_cpus::get());
        pool.for_(&courses, |course| {
            let support = calculate_support(&students, &course);
            if support >= desired_support {
                safe_next_level.lock().unwrap().push(course.clone());
            }
        });
        let mut next_level = safe_next_level.into_inner().unwrap();
        println!("Level {:?} complete! Found {:?} combinations with enough support.", level, next_level.len());
        if level == 4 {
            break;
        }
        level += 1;
        next_level.sort();
        courses = generate(next_level);
        println!("Starting to generate level {:?} candidates...", level);
    }
}

pub fn calculate_support(students: &[Student], courses: &[&Course]) -> f32 {
    let mut count = 0;
    for student in students {
        if courses.iter().all(|course| student.course_codes.contains(&course.code) ) {
            count += 1
        }
    }
    (count as f32 / students.len() as f32) as f32
}

pub fn generate<'a>(courses: Vec<Vec<&'a Course>>) -> Vec<Vec<&'a Course<'a>>> {
    let mut res = Vec::new();
    if courses.len() <= 1 {
        return res;
    }

    let mut index: usize = 0;
    while index < courses.len() - 1 {
        let first = &courses[index];
        let mut index2: usize = index + 1;
        while index2 < courses.len() {
            let second =  &courses[index2];
            if !adding_makes_sense(first, second) {
                break;
            }
            res.push(union(first, second));
            index2 += 1;
        }
        index += 1;
    }
    res
}

pub fn adding_makes_sense<'a>(first: &[&Course<'a>], second: &[&Course<'a>]) -> bool {
    if (first.len() != second.len()) || first.is_empty() {
        return false;
    }
    let end = first.len() - 1;
    first[0..end] == second[0..end]

}

pub fn union<'a>(first: &Vec<&'a Course<'a>>, second: &[&'a Course<'a>]) -> Vec<&'a Course<'a>> {
    let mut res = first.clone();
    for e in second {
        if !res.contains(e) {
            res.push(e);
        }
    }
    res
}
