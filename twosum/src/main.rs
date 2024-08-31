// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]
 

// Constraints:

// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.
 

// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

use std::collections::{VecDeque, HashMap};

struct Solution;
impl Solution {
    pub fn two_sum(&self, nums: Vec<i32>, target: i32) 
    -> Vec<i32> 
    {
        let mut deque1: VecDeque<i32>= VecDeque::new();
        let mut num_deque= VecDeque::from(nums.clone());
        let mut target_first = num_deque.pop_front().unwrap();
        let mut ind_first = 0;
        while !num_deque.is_empty() {
            let mut return_ind = 0;
            for (i, num) in num_deque.iter().enumerate() {
                if target_first + num == target {
                    return_ind = ind_first + (i+1);
                };
                // println!("ind first: {:?}", ind_first);
                // println!("i :{:?}", i);
                // println!("target first: {:?}", target_first);
                // println!("num :{:?}", num);
                // println!("return ind: {:?}", return_ind);
                // println!("num queue : {:?}", num_deque);
                

            }
            match return_ind {
                0 => {
                    target_first = {match num_deque.pop_front() {
                        Some(value) => value,
                        None => break
                    }
                    };
                    ind_first += 1;
                },
                _ => return vec![ind_first as i32, return_ind as i32],
            }    
        }
        

        vec![0, 0]}
        
    pub fn two_sum2(&self, nums:Vec<i32>, target:i32) -> Vec<i32>  {
        let nums_hashmap: HashMap<i32, usize> = nums.iter()
                                .enumerate()
                                .map(|(i, &v)| (v, i))
                                .collect();

        for (i, num) in nums.iter().enumerate() {
                println!("{:?}", num);
                println!("{:?}", i);
                let final_target =target - num;
                match nums_hashmap.get(&final_target) {
                    Some(v) =>  {
                        if i  == *v {
                            continue;
                        } else {
                        return vec![i as i32, *v as i32]}
                    },
                    None => continue
                }
            }
        return vec![0, 0]
        }
}


        

fn main() {
    let mut sol = Solution{};
    let answers = sol.two_sum2(vec![3,2,4], 6);
    println!("{:?}", answers);
    let answers = sol.two_sum2(vec![2,7,11, 15], 9);
    println!("{:?}", answers);
    let answers = sol.two_sum2(vec![3,3], 6);
    println!("{:?}", answers);
}

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]
 