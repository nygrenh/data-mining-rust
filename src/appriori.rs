use itertools::Itertools;
use std::sync::Mutex;
use std::collections::HashMap;

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
    let mut support_map: Mutex<&HashMap<&Vec<&Course>, Vec<&Student>>> = Mutex::new(&HashMap::new());
    while !courses.is_empty() {
        let safe_survivors: Mutex<Vec<Vec<&Course>>> = Mutex::new(Vec::new());
        let mut pool = simple_parallel::Pool::new(num_cpus::get());
        pool.for_(&courses, |course| {
            let support = hashed_support(&students, &course, &support_map);
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
        courses = prune(courses, &survivors);
        println!("{:?} candidates survived prune", courses.len());
        println!("Starting to generate level {:?} candidates...", level);
    }
}

pub fn prune<'a>(courses: Vec<Vec<&'a Course>>, prev: &Vec<Vec<&'a Course>>) -> Vec<Vec<&'a Course<'a>>> {
    let mut res = Vec::new();
    for course in courses {
        let mut all = true;
        for i in 0..(course.len()) {
            let mut test = course.clone();
            test.remove(i);
            if !prev.contains(&test) {
                all = false;
                break;
            }
        }
        if all {
            res.push(course);
        }
    }
    res
}

pub fn hashed_support<'a>(students: &'a [Student<'a>], courses: &'a Vec<&'a Course<'a>>, support_map: &'a Mutex<&'a HashMap<&'a Vec<&'a Course<'a>>, Vec<&'a Student<'a>>>>) -> f32
 {
    let end = courses.len() - 1;
    let key = courses[0..end].to_vec();

    let count: usize;
    if support_map.lock().unwrap().contains_key(&key) {
        let group = support_map.lock().unwrap().get(&key).unwrap();
        let last = courses[end];
        count = group.iter().filter(|student| student.course_codes.contains(&last.code)).count();
    } else {
        let supp = supportive_students(students, courses);
        support_map.lock().unwrap().insert(&key, supp);
        count = supp.len();
    }
    (count as f32 / students.len() as f32) as f32
}

pub fn supportive_students<'a>(students: &'a [Student], courses: &[&Course]) -> Vec<&'a Student<'a>> {
    let mut ret = Vec::new();
    for student in students {
        if courses.iter().all(|course| student.course_codes.contains(&course.code) ) {
            ret.push(student);
        }
    }
    ret
}

pub fn generate<'a>(courses: &Vec<Vec<&'a Course>>) -> Vec<Vec<&'a Course<'a>>> {
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
