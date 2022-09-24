use crate::algorithms::bubble_sort::bubble_sort_test;
use crate::algorithms::linear_search::linear_search_test;
use crate::algorithms::binary_search::binary_search_test;
use crate::algorithms::two_crystal_balls::two_crystal_balls_test;

pub mod algorithms;

fn main() {
    linear_search_test();
    binary_search_test();
    two_crystal_balls_test();
    bubble_sort_test();
    println!("tests successful");

}
