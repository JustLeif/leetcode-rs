use std::collections::HashMap;
pub fn solution(s: String, t: String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();

    for (s_ch, t_ch) in s.chars().zip(t.chars()) {
        *map.entry(s_ch).or_insert(0) += 1;
        *map.entry(t_ch).or_insert(0) -= 1;
    }

    for val in map.into_values() {
        if val != 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
#[test]
fn test_problem_242() {
    assert!(solution("val".to_string(), "lva".to_string()));
    assert!(solution("FFFFFsssAf".to_string(), "sFFsFFsFAf".to_string()));
    assert_eq!(solution("vFFFFal".to_string(), "lva".to_string()), false);
}
