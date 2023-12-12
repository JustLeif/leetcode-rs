/// You are given two non-empty linked lists representing two non-negative integers.
/// The digits are stored in reverse order, and each of their nodes contains a single digit.
/// Add the two numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
/// Example:
///
/// l1 = [2,4,3], l2 = [5,6,4]
///
/// 342 + 465 = 807
///
/// Output = [7,0,8]
pub fn solution(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut output = Box::new(ListNode::new(0));
    let mut curr = &mut output;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry > 0 {
        let mut total = 0;
        if let Some(node) = l1 {
            total += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            total += node.val;
            l2 = node.next;
        }
        carry = total / 10;
        total = total % 10;

        curr.next = Some(Box::new(ListNode::new(total)));
        curr = curr.next.as_mut().unwrap();
    }
    output.next
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
#[test]
fn test_2() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let expected_output = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 8, next: None })),
        })),
    }));

    assert_eq!(solution(l1, l2), expected_output);
}
