extern crate core;

use std::time::SystemTime;

use crate::arrays_strings::Solution;

mod arrays_strings;
mod utils;

fn main() {
    // println!("{:?}", Solution::product_except_self(vec![-1, 1, 0, -3, 3]));
    //
    // println!("{:?}", Solution::spiral_matrix(
    //     vec![
    //         vec![1, 2, 3],
    //         vec![4, 5, 6],
    //         vec![7, 8, 9],
    //     ]
    // ));
    //
    // println!("{:?}", Solution::four_sum_count(
    //     vec![0, 1, -1],
    //     vec![-1, 1, 0],
    //     vec![0, 0, 1],
    //     vec![-1, 1, 1],
    // ));

    // println!("{:?}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));


    Solution::game_of_life(
        &mut vec![
            vec![1, 1],
            vec![1, 0],
        ]
    );

    Solution::game_of_life(
        &mut vec![
            vec![1, 1],
            vec![1, 0],
        ]
    );

    Solution::game_of_life(
        &mut vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ]
    );
    //
    Solution::game_of_life(
        &mut vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ]
    );

    // println!("{}", Solution::missing_number(vec![3,0,1]));
    // println!("{}", Solution::find_duplicate(vec![3,1,3,4,2]));
    // println!("{}", Solution::first_missing_positive(vec![1, 2, 0]));
    // println!("{}", Solution::first_missing_positive(vec![3,4,-1,1]));
    // println!("{}", Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    // println!("{}", Solution::first_missing_positive(vec![1]));
    // println!("{}", Solution::first_missing_positive(vec![1, 3, 0]));
    // println!("{}", Solution::longest_consecutive(vec![1, 2, -1]));
    // println!("{}", Solution::longest_consecutive(vec![]));
    // println!("{}", Solution::longest_consecutive(vec![100, 4, 200, 201, 1, 3, 2, 0]));
    // println!("{}", Solution::longest_consecutive_imperative(vec![0, 1, 2, 4, 8, 5, 6, 7, 9, 3, 55, 88, 77, 99, 999999999]));
    // println!("{}", Solution::longest_consecutive_functional(vec![0, 1, 2, 4, 8, 5, 6, 7, 9, 3, 55, 88, 77, 99, 999999999]));


    // println!("{:?}", Solution::max_sliding_window_functional(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
    // println!("{:?}", Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));


    Solution::move_zeroes(&mut vec![0, 1, 0, 3, 12]);
    Solution::move_zeroes_stdlib(&mut vec![0, 1, 0, 3, 12]);
    println!("Done");
    println!("{:?}", Solution::remove_duplicates(&mut vec![1, 1, 2]));
    println!("{:?}", Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));
}


fn timeit<F: Fn() -> T, T>(f: F) {
    let start = SystemTime::now();
    f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Elapsed time: {}ms", duration.as_millis());
}

