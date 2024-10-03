#![allow(unused)]

use core::num;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut next = 0;
        let mut cur = 0;
        let mut step = 0;
        for i in 0..nums.len() {
            // 如果当前位置 i 超出了当前最远可到达位置 cur，说明需要进行一次跳跃
            if i > cur {
                cur = next;
                step += 1;
            }
            // 更新最远可到达位置
            next = std::cmp::max(next, i + nums[i] as usize);
        }
        step
    }
    pub fn can_jump(nums: Vec<i32>) -> bool {
        //  解题思路：
        // 维护一个变量 maxReach，表示当前能跳到的最远位置。
        // 遍历数组的每一个元素，判断当前下标是否可达（即判断当前下标是否小于等于 maxReach）。
        // 如果当前下标可达，更新 maxReach 为 i + nums[i]，即从当前位置能跳到的最远位置。
        // 如果遍历过程中 maxReach 已经大于等于最后一个下标，则返回 true。
        // 如果循环结束后 maxReach 仍未达到最后一个下标，返回 false。
        let mut max_reach = 0;
        for i in 0..nums.len(){
            if i > max_reach {
                return false
            }
            let next = i + nums[i] as usize;
            max_reach = std::cmp::max(max_reach, next);
            if max_reach >= nums.len() - 1 {
                return true
            }
        }
        false
    }
    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..prices.len(){
            ans += std::cmp::max(0, prices[i] - prices[i - 1])
        }
        ans
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 {
            return 0; // 如果数组为空，直接返回0
        }
    
        let mut dp = vec![0; n]; // 初始化dp数组，长度为n，所有值为0
        let mut min_profit = prices[0]; // 初始化最小价格
    
        for i in 1..n {
            min_profit = std::cmp::min(min_profit, prices[i]); // 更新最小价格
            dp[i] = std::cmp::max(dp[i-1], prices[i] - min_profit); // 更新dp值
        }
    
        dp[n - 1] // 返回最大利润
    }
    
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k % n as i32 + n as i32) % n as i32; // 取模并处理负数情况
        let mut rotated = vec![0; n]; // 创建一个新的数组
    
        for i in 0..n {
            rotated[((i as i32 + k) % n as i32) as usize] = nums[i]; // 计算新位置
        }
    
        *nums = rotated; // 将新数组赋值回原数组
    }
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count_map = HashMap::new();
        let threshold = nums.len() / 2;
    
        for &num in &nums {
            let count = count_map.entry(num).or_insert(0);
            *count += 1;
            if *count > threshold {
                return num; // 一旦找到了多数元素就返回
            }
        }
    
        // 由于题目保证存在多数元素，这里不会到达
        unreachable!()
    }
    pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
        match nums.len() {
            len if len < 3 => len as i32,
            _ => {
                let mut i = 2; // 第一个有效的位置
                let len = nums.len();
                for j in 2..len {
                    if nums[j] != nums[i - 2] {
                        nums[i] = nums[j];
                        i += 1;
                    }
                }
                i as i32
            },
        }
    }
    
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
        if nums.is_empty() {
            return 0;
        }

        let mut unique_index = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[unique_index] {
                unique_index += 1;
                nums[unique_index] = nums[i];
            }
        }

        unique_index + 1
    }
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
        let mut i = 0;

        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }

        i
    }
    pub fn maximum_length(s: String) -> i32 {
        let n = s.len();
        let mut max_length = -1;
        let mut count_map = HashMap::new();

        let mut i = 0;
        while i < n {
            let char = s.chars().nth(i).unwrap();
            let mut length = 0;

            while i < n && s.chars().nth(i).unwrap() == char {
                length += 1;
                i += 1;
            }

            if length > 1 {
                *count_map.entry(length).or_insert(0) += 1;
            }
        }

        for (&length, &count) in count_map.iter() {
            if count >= 3 {
                max_length = max_length.max(length);
            }
        }

        max_length
    }

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = (m + n - 1);

        while i >= 0 && j >= 0  {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            }else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        while j >= 0 {
            nums1[k as usize]  = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // test6();
    // test7();
    // test8();
    // test9();
    // test10();
    test11();
}

fn test11(){
    let mut nums = vec![2,3,1,1,4];
    let res = Solution::jump(nums);
    print!("{}",res);
}

fn  test10(){
    let nums = vec![2,3,1,1,4];
    let res = Solution::can_jump(nums);
    print!("{}",res);
}

fn test9(){
    let price = vec![7,1,5,3,6,4];
    let max_profit = Solution::max_profit_1(price);
    println!("{}",max_profit); 
}

fn test8(){
    let price = vec![7,1,5,3,6,4];
    let max_profit = Solution::max_profit(price);
    println!("{}",max_profit);
}
fn test7(){
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    Solution::rotate(&mut nums, k);
    println!("轮转后的数组: {:?}", nums);
}

fn test6(){
    let nums = vec![3,2,3];
    let num = Solution::majority_element(nums);
    print!("{}",num);
}

fn test5(){
    let mut nums = vec![1,1,1,2,2,3];
    let len = Solution::remove_duplicates_1(&mut nums);
    print!("{}",len);
}

fn test4(){
    let mut nums = vec![1,1,2];
    let len = Solution::remove_duplicates(&mut nums);
    print!("{}",len);
}

fn test3(){
    let mut arr = vec![3, 2,2,3];
    let val = 3;
    let index = Solution::remove_element(&mut arr, val);
    print!("{:?},{}",arr, index);

}

fn test2() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1); // 输出: [1, 2, 2, 3, 5, 6]
}

fn test1() {
    let s = String::from("aaaaaa");
    let result = Solution::maximum_length(s);
    println!("{}", result);
}
