pub mod solutions;

#[cfg(test)]
mod tests {

    #[test]
    fn solution_7() {
        use crate::solutions::problem_7::solution;
        assert_eq!(solution(123), 321);
        assert_eq!(solution(-123), -321);
        assert_eq!(solution(120), 21);
        assert_eq!(solution(0), 0);
        assert_eq!(solution(1534236469), 0);
    }

    #[test]
    fn solution_2() {
        use crate::solutions::problem_2::{solution, ListNode};

        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None}))}))}));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None}))}))}));
        let expected_output = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None}))}))}));

        assert_eq!(solution(l1, l2), expected_output);

    }
}
