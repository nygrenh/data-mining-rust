use itertools::Itertools;
use std::sync::Mutex;

extern crate simple_parallel;
extern crate num_cpus;
extern crate bit_vec;

use student::Student;
use student::Course;
use bit_vec::BitVec;

pub fn appriori(students: Vec<BitVec>,
                 desired_support: f32) {
    println!("Starting appriori with support {:?}", desired_support);
    println!("Generating level 1...");

    let courses_count = students[0].len();
    println!("courses_count {:?}", courses_count);
    let mut courses: Vec<BitVec> = Vec::new();
    for i in 0..courses_count {
        let mut vector = BitVec::from_elem(courses_count, false);
        vector.set(i, true);
        courses.push(vector)
    }

    println!("courses: {:?}", courses.len());

    println!("First level has been generated!");
    let mut level = 1;
    while !courses.is_empty() {
        let safe_survivors: Mutex<Vec<BitVec>> = Mutex::new(Vec::new());
        let mut pool = simple_parallel::Pool::new(num_cpus::get());
        pool.for_(&courses, |course| {
            let support = calculate_support(&students, &course);
            if support >= desired_support {
                safe_survivors.lock().unwrap().push(course.clone());
            }
        });
        let mut survivors = safe_survivors.into_inner().unwrap();
        println!("Level {:?} complete! Found {:?} combinations with enough support.", level, survivors.len());
        if level == 4 {
            break;
        }
        level += 1;
        survivors.sort();
        courses = generate(&survivors);
        println!("Generated {:?} candidates", courses.len());
        // courses = prune(courses, &survivors);
        // println!("{:?} candidates survived prune", courses.len());
        println!("Starting to generate level {:?} candidates...", level);
    }
}

// pub fn prune<'a>(courses: Vec<Vec<&'a Course>>, prev: &Vec<Vec<&'a Course>>) -> Vec<Vec<&'a Course<'a>>> {
//     let mut res = Vec::new();
//     for course in courses {
//         let mut all = true;
//         for i in 0..(course.len()) {
//             let mut test = course.clone();
//             test.remove(i);
//             if !prev.contains(&test) {
//                 all = false;
//                 break;
//             }
//         }
//         if all {
//             res.push(course);
//         }
//     }
//     res
// }
//
pub fn calculate_support(students: &Vec<BitVec>, courses: &BitVec) -> f32 {
    let mut count = 0;
    for student_vector in students {
        let mut diff = courses.clone();
        diff.difference(student_vector);
        if diff.none() {
            count += 1
        }
    }
    (count as f32 / students.len() as f32) as f32
}

pub fn generate<'a>(courses: &Vec<BitVec>) -> Vec<BitVec> {
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
                // println!("Didn't make sense: {:?} and {:?}", first, second);
                break;
            }

            res.push(union(first, second));
            index2 += 1;
        }
        index += 1;
    }
    res
}

pub fn adding_makes_sense<'a>(first: &BitVec, second: &BitVec) -> bool {
    // TODO: Do we need this?
    // if (first.len() != second.len()) || first.is_empty() {
    //     return false;
    // }
    let mut count = 0;
    let limit = first.iter().filter(|x| *x).count() - 1;
    for i in 0..(first.len() - 1) {
        if count >= limit {
            break;
        }
        if first.get(i).unwrap() != second.get(i).unwrap() {
            return false;
        }
        count += 1;
    }
    true
}

pub fn union<'a>(first: &BitVec, second: &BitVec) -> BitVec {
    let mut result = first.clone();
    result.union(second);
    result
}
