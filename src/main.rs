#![feature(plugin)]
#![plugin(clippy)]
#[macro_use] extern crate itertools;

mod student;
mod appriori;

use student::Student;

fn main() {
    let data = include_str!("data-2016.csv");
    let students = Student::create(data);
    appriori::appriori(&students, 0.05);
}
