// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV.
// Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
//  I can be placed before V (5) and X (10) to make 4 and 9.
//  X can be placed before L (50) and C (100) to make 40 and 90.
//  C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.
pub fn solution(input: String) -> i32 {
    let mut res = 0;
    let mut prev: Option<char> = None;
    for ch in input.chars() {
        match ch {
            'I' => res += 1,
            'V' => {
                if prev == Some('I') {
                    res += 3;
                } else {
                    res += 5;
                }
            }
            'X' => {
                if prev == Some('I') {
                    res += 8;
                } else {
                    res += 10;
                }
            }
            'L' => {
                if prev == Some('X') {
                    res += 30;
                } else {
                    res += 50;
                }
            }
            'C' => {
                if prev == Some('X') {
                    res += 80;
                } else {
                    res += 100;
                }
            }
            'D' => {
                if prev == Some('C') {
                    res += 300;
                } else {
                    res += 500;
                }
            }
            'M' => {
                if prev == Some('C') {
                    res += 800;
                } else {
                    res += 1000;
                }
            }
            _ => {}
        }
        prev = Some(ch);
    }
    res
}

#[cfg(test)]
#[test]
fn test_problem_13() {
    assert_eq!(solution(String::from("IV")), 4);
    assert_eq!(solution(String::from("MIV")), 1004);
    assert_eq!(solution(String::from("LVIII")), 58);
    assert_eq!(solution(String::from("MCMXCIV")), 1994);
}
