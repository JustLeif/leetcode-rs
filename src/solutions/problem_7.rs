/// Given a signed 32-bit integer x, return x with its digits reversed.
/// If reversing x causes the value to go outside the signed 32-bit integer
/// range [-231, 231 - 1], then return 0.
///
/// Example:
///
/// | 123 -> 321
///
/// | -123 -> -321
///
/// | 120 -> 21
///
/// | 1534236469 -> 0
///
/// Solution Explaination: https://www.youtube.com/watch?v=9ng9rMoT7T4&lc=UgwXtg1C4I1rgoYjzHB4AaABAg
pub fn solution(input: i32) -> i32 {
    return reverse(input).unwrap_or(0);
}

fn reverse(mut input: i32) -> Option<i32> {
    let mut output: i32 = 0;
    let mut temp: i32;
    while input != 0 {
        temp = input % 10; // 123 -> temp = 3
        input = input / 10; // 123 -> input = 12
        output = output.checked_mul(10)?.checked_add(temp)?;
    }
    return Some(output);
}

#[cfg(test)]
#[test]
fn solution_7() {
    assert_eq!(solution(123), 321);
    assert_eq!(solution(-123), -321);
    assert_eq!(solution(120), 21);
    assert_eq!(solution(0), 0);
    assert_eq!(solution(1534236469), 0);
}
