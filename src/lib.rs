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
}
