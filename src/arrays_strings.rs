use std::collections::VecDeque;

use crate::utils::Utils;

pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        /// https://leetcode.com/problems/product-of-array-except-self/
        let mut result = vec![1; nums.len()];

        let mut left = 1;
        for i in 1..nums.len() {
            left *= nums[i - 1];
            result[i] = left;
        }

        let mut right = 1;
        for j in (0..nums.len() - 1).rev() {
            right *= nums[j + 1];
            result[j] *= right
        }
        result
    }

    pub fn spiral_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        /// https://leetcode.com/problems/spiral-matrix/
        /// Pop and rotate the matrix
        let mut spiral: Vec<i32> = Vec::new();
        let mut matrix_deque: VecDeque<Vec<i32>> = VecDeque::from(matrix);

        while !matrix_deque.is_empty() {
            // Pop the top row of the matrix
            spiral.append(&mut matrix_deque.pop_front().unwrap());


            //Rotate the matrix
            for row in matrix_deque.iter_mut() {
                (*row).reverse()
            }
            matrix_deque = Utils::transpose(matrix_deque);
        }
        spiral
    }
}
