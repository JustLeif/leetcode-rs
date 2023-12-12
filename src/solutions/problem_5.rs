pub fn solution(input: String) -> String {
    let mut result = &input[0..0];
    println!("This is result: {}", result);
    for idx in 0..input.len() {
        // Loop to check the surrounding char
        let mut iter_idx: usize = 0;
        while input.chars().nth(idx + iter_idx) != None
            && idx - iter_idx > 0
            && input.chars().nth(idx + iter_idx) == input.chars().nth(idx - iter_idx)
        {
            result = &input[idx - iter_idx..idx + iter_idx];
            println!("this is res: {}", result);
            iter_idx += 1;
        }
    }

    return String::from(result);
}

#[cfg(test)]
#[test]
fn test_problem5() {
    assert_eq!(solution(String::from("abad")), String::from("aba"));
}
