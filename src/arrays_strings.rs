use std::collections::{HashMap, VecDeque};

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
    ///
    /// [0,1,3] -> 2
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        (((n + 1) * n) / 2) - nums.iter().sum::<i32>()
    }

    /// https://leetcode.com/problems/find-the-duplicate-number/
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut seen: Vec<bool> = std::iter::repeat(false).take(nums.len()).collect();

        for n in nums {
            if seen[n as usize] == true {
                return n;
            } else {
                seen[n as usize] = true
            }
        }

        println!("FAILED");
        return -1;
    }

    /// https://leetcode.com/explore/interview/card/top-interview-questions-hard/116/array-and-strings/832/
    ///Given an unsorted integer array nums, return the smallest missing positive integer.
    ///
    /// You must implement an algorithm that runs in O(n) time and uses constant extra space.
    ///
    ///
    ///
    /// Example 1:
    ///
    /// Input: nums = [1,2,0]
    /// Output: 3
    /// Explanation: The numbers in the range [1,2] are all in the array.
    /// Example 2:
    ///
    /// Input: nums = [3,4,-1,1]
    /// Output: 2
    /// Explanation: 1 is in the array but 2 is missing.
    /// Example 3:
    ///
    /// Input: nums = [7,8,9,11,12]
    /// Output: 1
    /// Explanation: The smallest positive integer 1 is missing.
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        use std::iter::repeat;
        let max_num = *nums.iter().max().unwrap();
        let array_size = (nums.len() as i32).max(max_num + 1);
        let mut seen: Vec<bool> = repeat(false).take(array_size as usize).collect();

        for &n in &nums {
            if n >= 0 {
                seen[n as usize] = true;
            }
        }
        seen[0] = true;

        seen.iter().position(|x| !*x).unwrap_or(*&nums.len() + 1) as i32
    }

    /// https://leetcode.com/explore/interview/card/top-interview-questions-hard/116/array-and-strings/833/
    /// Generate a set then traverse counting the longest consecutive run
    pub fn longest_consecutive_imperative(nums: Vec<i32>) -> i32 {
        let mut set: std::collections::HashSet<i32> = nums.into_iter().collect();

        let mut current = 0;
        let mut longest = 0;

        for n in &set {
            if set.contains(&(n - 1)) {
                continue;
            }

            current = 1;

            let mut up = n + 1;
            while set.contains(&up) {
                current += 1;
                up += 1;
            }

            longest = longest.max(current);
        }
        longest
    }

    /// https://leetcode.com/explore/interview/card/top-interview-questions-hard/116/array-and-strings/833/
    /// Generate a set then traverse counting the longest consecutive run
    pub fn longest_consecutive_functional(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let seq: HashSet<i32> = nums.into_iter().collect::<HashSet<i32>>();

        seq.iter()
            .filter(|&&x| x != i32::MIN && !seq.contains(&(x - 1)))
            .map(|&x| (x..=i32::MAX).take_while(|x| seq.contains(x)).count())
            .max()
            .unwrap_or(0) as i32
    }

    pub fn max_sliding_window_functional(nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.windows(k as usize)
            .map(|window| *window.iter().max().unwrap())
            .collect()
    }

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut deq: VecDeque<i32> = VecDeque::with_capacity(k as usize);
        let mut maxes: Vec<i32> = Vec::new();

        let mut rolling_max = i32::MIN;

        for n in nums {
            deq.push_front(n);

            let mut popped  = i32::MIN;
            if deq.len() as i32 > k {
                popped = deq.pop_back().unwrap();
            }

            // println!("Popped: {popped}, rolling_max: {rolling_max}, n: {n}");
            if deq.len() as i32 == k {
                // n, rolling_max, popped
                if n > rolling_max {
                    rolling_max = n
                } else if popped < n && n < rolling_max {
                    // If n is less than the old rolling_max
                    // and the old maximum wasn't popped off the end then
                    // there's no need to compute a new rolling_max
                } else if n < rolling_max && popped < rolling_max {
                    // also no need to update
                } else {
                    rolling_max = n.max(*deq.iter().max().unwrap());
                }
                maxes.push(rolling_max);
            }
        }
        maxes
    }

    /// [0,1,0,3,12] -> [1,3,12,0,0]
    /// [1,2,0,4,0,5] -> [1,2,4,5,0,0]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // walk along finding non zero numbers
        // swap non-zero number position after last non-zero
        // done.

        let mut last_non_zero = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, last_non_zero);
                last_non_zero += 1;
            }
        }
    }

    /// Same as above but cheat using the standard library
    pub fn move_zeroes_stdlib(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|x| *x != 0);
        nums.resize(len, 0);
    }
}