use std::collections::HashMap;

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.
pub fn solution(nums: Vec<i32>, target: i32) -> [i32; 2] {
    let mut map = HashMap::<i32, usize>::new();
    for (idx, val) in nums.iter().enumerate() {
        map.insert(val.clone(), idx);
    }

    for (fst_idx, val) in nums.iter().enumerate() {
        let diff = target - val.clone();
        if let Some(scnd_idx) = map.get(&diff) {
            return [fst_idx.clone() as i32, scnd_idx.clone() as i32];
        }
    }

    return [0, 0];
}

#[test]
fn test_problem_1() {
    assert_eq!(solution(vec![0, 49, 29, 39, 329], 329), [0, 4]);
}
