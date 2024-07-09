#![allow(unused)]

#[derive(Debug, PartialEq)]
pub enum MedianType {
    Int(i32),
    Float(f64),
}

fn merge_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut merged_vec= a.clone();
    merged_vec.extend(b);
    merged_vec.sort();
    merged_vec
}


pub fn find_median_of_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> MedianType {
    let length_combined = a.len() + b.len();
    let merged_vec = merge_sorted_arrays(a, b);
    let middle = (length_combined / 2);
    if (length_combined % 2 == 0) {
        let median = (merged_vec[middle - 1] + merged_vec[middle]) as f64 / 2.0;
        return MedianType::Float(median)
    } else {
        return MedianType::Int(merged_vec[middle]);
    }
}


pub fn recursive_find_median_of_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> MedianType {
    let a_len = a.len();
    let b_len = b.len();
    if (a_len > b_len) {
        return recursive_find_median_of_sorted_arrays(b, a)
    } 
    let comb_len = a_len + b_len;
    let left_part = (a_len + b_len + 1) / 2;
    let low = 0;
    let high = a_len;
    while low <= high {
        let mid1 = (low + high) / 2;
        let mid2 = left_part - mid1;
        let l1 = -1000;
        let l2 = 1000;
        let r1 = -1000;
        let r2 = 1000;
        if mid1 < a_len {
            
        }
        if mid2 < b_len {
            
        }
        if mid1 - 1 >= 0 {
            
        }
        if mid2 - 1 >= 0 {
            
        }
        if (l1 <= r2 && l2 <= r1) {
            if comb_len % 2 == 1 {

            } else {
                
            }
        } else if  {
            
        } else {
            
        }
    }
    return MedianType::Int(0);
}
