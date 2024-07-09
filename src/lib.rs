mod utils;

#[cfg(test)]
mod tests {
    use super::utils::{add_numbers, sub_numbers};

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
    }
    
    #[test]
    fn test_sub_numbers() {
        assert_eq!(sub_numbers(2, 3), -1);
    }
}

