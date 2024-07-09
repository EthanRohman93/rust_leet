mod utils;

fn main() {
    let sum = utils::add_numbers(2, 3);
    let diff = utils::sub_numbers(2, 3);
    println!("Sum: {}, Difference: {}", sum, diff);
}
