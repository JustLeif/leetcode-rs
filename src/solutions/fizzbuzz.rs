use std::collections::HashMap;

pub fn solution_with_match(range: i32) {
    for index in 0..range {
        match (index % 3, index % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            (_, _) => {}
        }
    }
}

pub fn solution_with_hashmap(range: i32) {
    let hash_map = HashMap::from([
        (3, "fizz"),
        (5, "buzz"),
        // Add more combinations here...
        // (6, "divis"),
        // (10, "byten")
    ]);

    for index in 0..range {
        let mut output = String::from("");
        for key in hash_map.keys() {
            if index % key == 0 {
                output.insert_str(output.len(), hash_map.get(key).expect("Error obtaining expected key!"));
            }
        }
        if output != String::from("") {
            println!("{}", output);
        }
    }
}

#[cfg(test)]
#[test]
fn test_fizzbuzz() {
    use crate::solutions::fizzbuzz;
    fizzbuzz::solution_with_match(101);
    fizzbuzz::solution_with_hashmap(101);
}