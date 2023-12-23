pub fn solution(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l: usize = 0;
    let mut r: usize = numbers.len() - 1;

    while l < r {
        if numbers[l] + numbers[r] == target {
            return vec![l as i32, r as i32];
        } else if numbers[l] + numbers[r] > target {
            r -= 1;
        } else if numbers[l] + numbers[r] < target {
            l += 1;
        }
    }
    unreachable!();
}

#[test]
pub fn test_167() {
    let res = solution(vec![4, 5, 6, 7, 8, 9, 20], 26);
    assert_eq!(res, vec![2, 6]);
}
