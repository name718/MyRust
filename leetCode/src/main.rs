use std::collections::HashMap;

struct Solution;

impl Solution {
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
    test1();
    test2();
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
