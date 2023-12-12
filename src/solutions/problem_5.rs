pub fn solution(input: String) -> String {
    let mut left: usize = 0;
    let mut right: usize = 0;

    let c_vec: Vec<char> = input.chars().collect();
    for idx in 0..c_vec.len() {
        let mut tl: i32 = idx as i32 - 1;
        let mut tr: i32 = idx as i32 + 1;

        while tl >= 0 && (tr as usize) < c_vec.len() && c_vec[tl as usize] == c_vec[tr as usize] {
            if tr as usize - tl as usize > right - left {
                left = tl as usize;
                right = tr as usize;
            }
            tl -= 1;
            tr += 1;
        }
    }

    return input[left..=right].to_string();
}

#[cfg(test)]
#[test]
fn test_problem5() {
    assert_eq!(solution(String::from("abad")), String::from("aba"));
    assert_eq!(
        solution(String::from("aaaaaaaabad")),
        String::from("aaaaaaa")
    );
    assert_eq!(solution(String::from("ftftttf")), String::from("ftttf"));
}
