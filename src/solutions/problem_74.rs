/**
 * You are given an m x n integer matrix matrix with the following two properties:
 *      Each row is sorted in non-decreasing order.
 *      The first integer of each row is greater than the last integer of the previous row.
 * Given an integer target, return true if target is in matrix or false otherwise.
 * You must write a solution in O(log(m * n)) time complexity.
 */
pub fn solution(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // Perform an outer and inner binary search.
    let mut outer_low: usize = 0;
    let mut outer_high: usize = matrix.len();
    let mut outer_mid: usize;

    while outer_low < outer_high {
        outer_mid = outer_low + (outer_high - outer_low) / 2;

        // We can use a range check on the midpoint vec, to see if we even want to enter it.
        if &target >= &matrix[outer_mid][0]
            && &target <= &matrix[outer_mid][matrix[outer_mid].len() - 1]
        {
            // Perform a binary search on the INNER vec
            let mut inner_low: usize = 0;
            let mut inner_high: usize = matrix[outer_mid].len();
            let mut inner_mid: usize;

            while inner_low < inner_high {
                inner_mid = inner_low + (inner_high - inner_low) / 2;

                if &target == &matrix[outer_mid][inner_mid] {
                    return true;
                } else if &target < &matrix[outer_mid][inner_mid] {
                    inner_high = inner_mid;
                } else if &target > &matrix[outer_mid][inner_mid] {
                    inner_low = inner_mid + 1;
                }
            }

            // If we range checked, and the inner binary search was unsuccessful the value is not in there.
            return false;
        } else if &target < &matrix[outer_mid][0] {
            outer_high = outer_mid;
        } else if &target > &matrix[outer_mid][matrix[outer_mid].len() - 1] {
            outer_low = outer_mid + 1;
        }
    }
    return false;
}

#[cfg(test)]
#[test]
pub fn test_problem_74() {
    let matrix = vec![
        vec![22, 45, 67, 89],
        vec![100, 388, 500, 600],
        vec![800, 900, 1000, 1100],
        vec![1200, 1300, 4000, 5684],
    ];
    assert_eq!(solution(matrix, 1200), true);

    let matrix = vec![
        vec![22, 45, 67, 89],
        vec![100, 388, 500, 600],
        vec![800, 900, 1000, 1100],
        vec![1200, 1300, 4000, 5684],
    ];
    assert_eq!(solution(matrix, 108488), false);
}
