use crate::arrays_strings::Solution;

mod arrays_strings;
mod utils;

fn main() {
    println!("{:?}", Solution::product_except_self(vec![-1, 1, 0, -3, 3]));

    println!("{:?}", Solution::spiral_matrix(
        vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]
    ));
}

