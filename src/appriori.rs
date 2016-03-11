use std::sync::Mutex;

extern crate simple_parallel;
extern crate num_cpus;
extern crate bit_set;

use bit_set::BitSet;
use std::collections::HashSet;


pub fn appriori(students: Vec<BitSet>, desired_support: f32, number_of_courses: usize) {
    println!("Starting appriori with support {:?}", desired_support);
    println!("Generating level 1...");

    let mut courses: Vec<BitSet> = Vec::new();
    for i in 0..number_of_courses {
        let mut vector = BitSet::with_capacity(number_of_courses);
        vector.insert(i);
        courses.push(vector)
    }
    courses = generate(&courses);
    println!("First level has been generated!");

    let mut level = 2;
    while !courses.is_empty() {
        let safe_survivors: Mutex<Vec<BitSet>> = Mutex::new(Vec::new());
        let mut pool = simple_parallel::Pool::new(num_cpus::get());
        pool.for_(&courses, |course| {
            let support = calculate_support(&students, &course);
            // println!("support: {:?}", support);
            if support >= desired_support {
                safe_survivors.lock().unwrap().push(course.clone());
            }
        });
        let mut survivors = safe_survivors.into_inner().unwrap();
        println!("Level {:?} complete! Found {:?} combinations with enough support.",
                 level,
                 survivors.len());
        if level == 17 {
            println!("The longest course combinations are: {:?}", survivors);
            return;
        }
        level += 1;
        survivors.sort();
        // println!("Survivors: {:?}", survivors);
        println!("Starting to generate level {:?} candidates...", level);
        courses = generate(&survivors);
        // println!("courses: {:?}", courses);
        // println!("Generated: {:?}", courses);1
        println!("Generated {:?} candidates", courses.len());
        // courses = prune(&courses, &survivors);
        // println!("{:?} candidates survived prune", courses.len());

    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn prune(courses: &[BitSet], prev: &Vec<BitSet>) -> Vec<BitSet> {
    let hash_set: HashSet<BitSet> = prev.clone().into_iter().collect();
    let mut pool = simple_parallel::Pool::new(num_cpus::get());
    let res: Mutex<Vec<BitSet>> = Mutex::new(Vec::new());
    pool.for_(courses, |course| {
        let mut all = true;

        for item in course.iter().take(course.len()) {
            let mut test = course.clone();
            test.remove(item);
            if !hash_set.contains(&test) {
                all = false;
                break;
            }
        }
        if all {
            res.lock().unwrap().push(course.clone());
        }
    });
    res.into_inner().unwrap()
}

#[inline(always)]
pub fn calculate_support(students: &[BitSet], courses: &BitSet) -> f32 {
    let count = students.iter().filter(|s| courses.is_subset(s)).count();
    (count as f32 / students.len() as f32) as f32
}

#[inline(always)]
pub fn generate(courses: &[BitSet]) -> Vec<BitSet> {
    let mut res = Vec::new();
    if courses.len() <= 1 {
        return res;
    }

    let mut index: usize = 0;
    while index < courses.len() - 1 {
        let first = &courses[index];
        let mut index2: usize = index + 1;
        while index2 < courses.len() {
            let second = &courses[index2];
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

#[inline(always)]
pub fn adding_makes_sense(first: &BitSet, second: &BitSet) -> bool {
    if (first.len() != second.len()) || first.is_empty() {
        return false;
    }

    let second_contents: Vec<usize> = second.iter().collect();
    let mut prev = false;
    for (index, b) in first.iter().enumerate() {
        let b2 = second_contents[index];
        if prev {
            return false;
        }
        if b != b2 {
            prev = true;
        }
    }
    true
}

#[inline(always)]
pub fn union(first: &BitSet, second: &BitSet) -> BitSet {
    first.union(second).collect()
}
