use std::fmt::Debug;

// runtime of O(n)
pub fn linear_search(arr: Vec<isize>, val: isize) -> isize {
    for (idx, value) in arr.iter().enumerate() {
        if value == &val {
            return idx as isize;
        }
    }
    return -1;
}

// Pass in a sorted vector to obtain an index of val in O(log n) time. Recieve -1 if val is not in arr.
pub fn binary_search(arr: Vec<isize>, val: isize) -> isize {
    let mut low: usize = 0;
    let mut high = arr.len();
    let mut mid;

    while low < high {
        mid = low + (high - low) / 2;
        if arr.get(mid) == Some(&val) {
            return mid as isize;
        } else if arr.get(mid) > Some(&val) {
            high = mid;
        } else if arr.get(mid) < Some(&val) {
            low = mid + 1;
        }
    }

    return -1;
}

#[cfg(test)]
#[test]
fn test_binary_search() {
    let test_arr: Vec<isize> = vec![0, 10, 30, 40, 50, 60, 100];
    assert_eq!(binary_search(test_arr, 50), 4);
    let test_arr_2: Vec<isize> = vec![0, 0, 10, 20, 22, 1230, 10000];
    assert_eq!(binary_search(test_arr_2, 1428), -1);
}

// Given a vec of booleans sorted by false -> true, you have two chances to find the exact position where it starts to be true. Do this in a O(sqrt(n)) runtime.
pub fn crystal_ball_search(arr: Vec<bool>) -> isize {
    let jmp_amount = f64::sqrt(arr.len() as f64) as usize;
    let mut i: usize = 0;

    while i < arr.len() && (i + jmp_amount) < arr.len() {
        if arr[i] == true {
            break;
        }
        i += jmp_amount;
    }

    i -= jmp_amount;

    while i <= i + jmp_amount && i < arr.len() {
        if arr[i] == true {
            return i as isize;
        }
        i += 1;
    }

    return -1;
}

#[cfg(test)]
#[test]
fn test_crystal_ball_search() {
    let test_arr = vec![false, false, false, false, false, true];
    assert_eq!(crystal_ball_search(test_arr), 5);
    let test_arr = vec![false, false];
    assert_eq!(crystal_ball_search(test_arr), -1);
    let test_arr = vec![false, true, true, true, true];
    assert_eq!(crystal_ball_search(test_arr), 1);
}

pub fn bubble_sort(arr: &mut Vec<usize>) {
    for i in 0..arr.len() {
        for j in 0..(arr.len() - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_bubble_sort() {
    let mut arr: Vec<usize> = vec![0, 20, 10, 5, 6, 6, 4];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec!(0, 4, 5, 6, 6, 10, 20));
    let mut arr: Vec<usize> = vec![1000, 492, 129, 5, 6, 6, 4];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec!(4, 5, 6, 6, 129, 492, 1000));
}

pub fn array_stuff<T: Debug>(arr: &mut [T; 5]) {
    println!("{:?}", arr);
}

#[cfg(test)]
#[test]
fn test_array() {
    let mut arr: [&str; 5] = ["val", "val", "notval", "unique", "stuff"];
    array_stuff(&mut arr);
    arr = ["Value?"; 5];
    array_stuff(&mut arr);
    println!("{:?}", 0..arr.len());
}

// Note to self, never implement a linked list in Rust !
