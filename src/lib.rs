#![allow(unused)]
mod utils;
mod median_of_two_sorted_arrays;
mod reverse_nodes_in_k_group;
mod merge_k_sorted_lists;

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::{add_numbers, sub_numbers};
    use super::median_of_two_sorted_arrays::{find_median_of_sorted_arrays, MedianType};
    use super::reverse_nodes_in_k_group::reverse_k_group;
    use super::merge_k_sorted_lists::merge_k_lists;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
    }
    
    #[test]
    fn test_sub_numbers() {
        assert_eq!(sub_numbers(2, 3), -1);
    }

    #[test]
    fn test_find_median_sorted_arrays() {
        let vec1 = vec![1, 3];
        let vec2 = vec![2, 4];
        assert_eq!(find_median_of_sorted_arrays(vec1, vec2), MedianType::Float(2.5))
    }

    #[test]
    fn test_reverse_nodes_in_k_group() {
        
    }

    #[test]
    fn test_merge_k_sorted_lists() {

    }
}

