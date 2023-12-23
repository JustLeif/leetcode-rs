pub fn solution(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            '}' => {
                if stack.last() != Some(&'{') {
                    return false;
                } else {
                    stack.pop();
                }
            }
            ']' => {
                if stack.last() != Some(&'[') {
                    return false;
                } else {
                    stack.pop();
                }
            }
            ')' => {
                if stack.last() != Some(&'(') {
                    return false;
                } else {
                    stack.pop();
                }
            }
            _ => {}
        }
    }

    return true;
}

#[test]
pub fn test_20() {
    let res = solution("()()()(){}".to_string());
    assert_eq!(true, res);

    let res = solution("((()))".to_string());
    assert_eq!(true, res);

    let res = solution("({}}".to_string());
    assert_eq!(false, res);
}
