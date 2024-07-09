#![allow(unused)]

#[derive(PartialEq)]
#[derive(Debug)]
pub enum MedianType {
    Int(i32),
    Float(f64),
}


pub fn find_median_of_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> MedianType {
    let x = MedianType::Int(2);
    let y = MedianType::Float(2.5);
    return x;
}
