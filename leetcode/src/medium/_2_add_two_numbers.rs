#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::_add_digits(l1, l2, 0)
    }

    fn _add_digits(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                if carry == 0 {
                    None
                } else {
                    Some(Box::new(ListNode::new(carry)))
                }
            }
            (Some(l1), None) => {
                let sum = l1.val + carry;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::_add_digits(l1.next, None, sum / 10),
                }))
            }
            (None, Some(l2)) => {
                let sum = l2.val + carry;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::_add_digits(None, l2.next, sum / 10),
                }))
            }
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val + carry;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::_add_digits(l1.next, l2.next, sum / 10),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    trait ListConverter {
        fn from_list(list: &[i32]) -> Option<Box<ListNode>>;
        fn to_list(&self) -> Vec<i32>;
    }

    impl ListConverter for ListNode {
        fn from_list(list: &[i32]) -> Option<Box<ListNode>> {
            let mut head = None;
            for &val in list.iter().rev() {
                let mut node = ListNode::new(val);
                node.next = head;
                head = Some(Box::new(node));
            }
            head
        }

        fn to_list(&self) -> Vec<i32> {
            let mut list = vec![];
            let mut node = self;
            while let Some(next) = &node.next {
                list.push(node.val);
                node = next;
            }
            list.push(node.val);
            list
        }
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_list(&[9, 9, 9, 9, 9, 9, 9]),
                ListNode::from_list(&[9, 9, 9, 9])
            )
            .unwrap()
            .to_list(),
            vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}
