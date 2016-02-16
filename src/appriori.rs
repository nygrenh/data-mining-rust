use itertools::Itertools;

use student::Student;
use student::Course;

pub fn appriori(students: &Vec<Student>,
                 desired_support: f32) {
    println!("Generating level 1...");
    let courses: Vec<Vec<u32>> = students.iter().flat_map(|student| &student.course_codes ).unique().map(|x| {
        let mut inner = Vec::new();
        inner.push(*x);
        inner
    }).collect();
    println!("Fist level: {:?}", courses);
    let mut courses: Vec<&Vec<Course>> = Vec::new();
    let mut level = 1;
    while !courses.is_empty() {
        let mut next_level: Vec<&Vec<Course>> = Vec::new();
        println!("Starting level {:?}...", level);
        for course in &courses {
            let support = calculate_support(&students, &course);
            if support <= desired_support {
                next_level.push(course);
            }
        }
        println!("Level {:?} complete! Found {:?} combinations with enough support.", level, next_level.len());
        level += 1;
        courses = next_level;
        println!("Starting to generate level {:?} candidates...", level);

    }
}

pub fn calculate_support(students: &[Student], courses: &[Course]) -> f32 {
    let mut count = 0;
    for student in students {
        if courses.iter().all(|course| student.course_codes.contains(&course.code) ) {
            count += 1
        }
    }
    (count / students.len()) as f32
}
