use std::collections::HashSet;

// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
pub fn solution(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    for val in nums.iter() {
        if let Some(_) = set.get(val) {
            return true;
        }
        set.insert(*val);
    }

    return false;
}
