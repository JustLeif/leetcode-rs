//Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
//You must write an algorithm with O(log n) runtime complexity.
pub fn solution(nums: Vec<i32>, target: i32) -> i32 {
    let mut low: usize = 0;
    let mut high = nums.len();
    let mut mid;

    while low < high {
        mid = low + (high - low) / 2;

        if nums.get(mid) == Some(&target) {
            return mid as i32;
        } else if nums.get(mid) > Some(&target) {
            high = mid;
        } else if nums.get(mid) < Some(&target) {
            low = mid + 1;
        }
    }

    return high as i32;
}

#[cfg(test)]
#[test]
pub fn test_35() {
    let vec: Vec<i32> = vec![1, 3, 5, 6, 7];
    assert_eq!(solution(vec, 4), 2);
    let vec: Vec<i32> = vec![1, 3, 5, 6, 7];
    assert_eq!(solution(vec, 5), 2);
}
