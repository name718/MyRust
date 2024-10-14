#![allow(unused)]
extern crate rand;

use core::num;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;
struct RandomizedSet {
    set:HashSet<i32>,
    vec:Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet{
            set:HashSet::new(),
            vec:Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.set.contains(&val) {
            return false;
        }
        self.set.insert(val);
        self.vec.push(val);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if self.set.contains(&val) {
            self.set.remove(&val);
            if let Some(index) = self.vec.iter().position(|&v| v == val) {
                self.vec.swap_remove(index);
            }
            return true;
        }
        false
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.vec.len());
        self.vec[random_index]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(x) => {
                x as i32
            }
            None => {
                -1
            }
        }
    }
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut rows = vec![String::new(); num_rows as usize];
        let mut current_row = 0;
        let mut going_down = false;
        for c in s.chars() {
            rows[current_row as usize].push(c);
            if current_row == 0 || current_row == num_rows - 1 {
                going_down =!going_down;
            }
            if going_down {
                current_row += 1;
            } else {
                current_row -= 1;
            }
        }
        rows.into_iter().collect()
    }
    pub fn reverse_words(s: String) -> String {
        let mut words:Vec<&str> = s.split_whitespace().collect();
        words.reverse();
        words.join(" ")
    }
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let mut prefix:String = String::from(&strs[0]);
    
        for str in strs.iter() {
            // println!("first:{}", str);
            while !str.starts_with(&prefix) {
                // println!("sencod:{}", prefix);
                prefix.pop();
                if prefix.is_empty() {
                    return String::new();
                }
            }
        }
        // println!("return:{}", prefix);
        return prefix;
    }
    fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.is_empty() {
            return 0;
        }
        words.last().unwrap().len() as i32
    }
    fn int_to_roman(num: i32) -> String {
        let value_symbols = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut n = num.clone();
        let mut roman = String::new();
        for (value, symbol) in value_symbols.iter() {
            while n >= *value {
                n -= *value;
                roman.push_str(symbol);
            }
            if n == 0 {
                break;
            }
        }
        roman
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut number = 0;
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);
    
        let ss: Vec<char> = s.chars().collect();
        for i in 0..ss.len() {
            let current_val: i32 = *map.get(&ss[i]).unwrap();
            let next_val: i32 = if i + 1 < ss.len() {
                *map.get(&ss[i + 1]).unwrap()
            } else {
                0
            };
            if current_val < next_val {
                number -= current_val
            } else {
                number += current_val
            }
        }
        println!("{}", number);
        return number;
    }
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut total_gas = 0;
        let mut total_cost = 0;
        let mut current_gas = 0;
        let mut start_station = 0;
    
        // 计算总油量和总消耗
        for i in 0..n {
            total_gas += gas[i];
            total_cost += cost[i];
        }
    
        // 如果总油量小于总消耗，无法绕一圈，返回 -1
        if total_gas < total_cost {
            return -1;
        }
    
        // 遍历每个加油站，尝试找到起始加油站
        for i in 0..n {
            current_gas += gas[i] - cost[i];
            if current_gas < 0 {
                // 如果当前油量不足，从下一个加油站重新开始尝试
                current_gas = 0;
                start_station = i + 1;
            }
        }
    
        start_station as i32
    }
    fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        // 创建一个初始值全为 1 的结果向量
        let mut answer = vec![1; n];
        // 计算每个元素左边所有元素的乘积
        let mut left_product = 1;
        for i in 0..n {
            // 将当前位置的结果更新为左边所有元素的乘积
            answer[i] *= left_product;
            // 更新左边元素的乘积
            left_product *= nums[i];
        }
        // 计算每个元素右边所有元素的乘积
        let mut right_product = 1;
        for i in (0..n).rev() {
            // 将当前位置的结果更新为左边乘积乘上右边乘积
            answer[i] *= right_product;
            // 更新右边元素的乘积
            right_product *= nums[i];
        }
        // 返回结果向量
        answer
    }
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // 对引用次数数组进行降序排序
        let mut sorted_citations = citations.clone();
        sorted_citations.sort_unstable_by(|a, b| b.cmp(a));

        let mut h = 0;
        // 遍历排序后的数组
        for (i, citation) in sorted_citations.iter().enumerate() {
            // 如果当前论文的引用次数大于等于当前的索引加一，说明至少有 i + 1 篇论文被引用了至少 i + 1 次
            if *citation >= (i as i32 + 1) {
                h = i as i32 + 1;
            } else {
                // 不满足条件时退出循环
                break;
            }
        }
        // 返回 H 指数
        h
    }
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
    // test11();
    // test12();
    // test13();
    // test14();
    // test15();
    // test16();
    // test17();
    // test18();
    // test19();
    test21();
}

fn test22(){
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");
    let mut num_rows = 3;
    let res = Solution::str_str(haystack,needle);
    print!("{}",res); 
}

fn test21(){
    let mut s = String::from("PAYPALISHIRING");
    let mut num_rows = 3;
    let res = Solution::convert(s,num_rows);
    print!("{}",res); 
}

fn test20(){
    let strs = String::from("the sky is blue");
    let res = Solution::reverse_words(strs);
    print!("{}",res); 
}

fn test19(){
    let strs = vec![String::from("flower"),String::from("flow"),String::from("flight")];
    let res = Solution::longest_common_prefix(strs);
    print!("{}",res); 
}

fn test18(){
    let s = String::from("Hello World");
    let res = Solution::length_of_last_word(s);
    print!("{}",res); 
}

fn test17(){
    let s = 4321;
    let res = Solution::int_to_roman(s);
    print!("{}",res); 
}

fn test16(){
    let s = String::from("III");
    let res = Solution::roman_to_int(s);
    print!("{}",res); 
}

fn test15(){
    let gas = vec![1,2,3,4,5];
    let cost = vec![3,4,5,1,2];
    let res = Solution::can_complete_circuit(gas,cost);
    print!("{}",res);
}

fn test14() {
    let nums = vec![1,2,3,4];
    let res = Solution::product_except_self(nums);
    print!("{:?}",res);
}

fn test13(){
    let mut obj = RandomizedSet::new();
    let ret_1: bool = obj.insert(1);
    let ret_2: bool = obj.remove(1);
    let ret_3: i32 = obj.get_random();
}

fn test12(){
    let citations = vec![3,0,6,1,5];
    let res = Solution::h_index(citations);
    print!("{}",res);
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
