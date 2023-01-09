use std::collections::{HashMap, VecDeque};
use std::env::join_paths;

use crate::utils::Utils;

pub struct Solution {}

impl Solution {
    /// https://leetcode.com/problems/product-of-array-except-self/
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
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

    /// https://leetcode.com/problems/spiral-matrix/
    /// Pop and rotate the matrix
    pub fn spiral_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
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

    /// Given four integer arrays nums1, nums2, nums3, and nums4 all of length n, return the number of tuples (i, j, k, l) such that:
    ///
    /// ```
    /// 0 <= i, j, k, l < n
    /// nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
    /// ```
    ///
    /// Solution: Create a count-up hashmap and then count-down using the same hashmap.
    /// O(len(nums)^2)
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for a in &nums1 {
            for b in &nums2 {
                counts.entry(a + b).and_modify(|x| *x += 1).or_insert(1);
            }
        }

        let mut score = 0;

        for c in &nums3 {
            for d in &nums4 {
                if let Some(value) = counts.get(&(-c - d)) {
                    score += value;
                }
            }
        }

        score
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;

        let mut i = 0;
        let mut j = height.len() - 1;

        let mut water = (j - i) * cmp::min(height[i], height[j]) as usize;

        while i < j {
            water = cmp::max(water, (j - i) * cmp::min(height[i], height[j]) as usize);

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        water as i32
    }

    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let frozen_board = board.clone();

        let directions: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let board_length = board.len();
        let board_width = board[0].len();

        for i in 0..board_length {
            for j in 0..board_width {

                let score: i32 = directions
                    .iter()
                    .map(|(a, b)| (a + i as i32, b + j as i32))
                    .filter(|(a, b)| *a >= 0 && *b >= 0 && *a < board_length as i32 && *b < board_width as i32)
                    .map(|(a, b)| frozen_board[a as usize][b as usize])
                    .sum();

                let alive = frozen_board[i][j];

                let new = if alive == 1 {
                    match score {
                        2..=3 => 1,
                        _ => 0
                    }
                } else {
                    match score {
                        3..=3 => 1,
                        _ => 0
                    }
                };

                board[i][j] = new;
            }
        }
        // println!("{:?}", board);
    }
}
