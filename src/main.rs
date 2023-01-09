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
}

