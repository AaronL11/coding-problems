impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        recursion(0, l1, l2)
    }
}

fn recursion(
    carry: i32,
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if let (0, None, None) = (carry, &l1, &l2) {
        return None;
    }
    let (v1, n1) = if let Some(list1) = l1 {
        (list1.val, list1.next)
    } else {
        (0, None)
    };
    let (v2, n2) = if let Some(list2) = l2 {
        (list2.val, list2.next)
    } else {
        (0, None)
    };
    let tot = v1 + v2 + carry;
    let (rem, carry) = (tot % 10, tot / 10);
    Some(Box::new(ListNode {
        val: rem,
        next: recursion(carry, n1, n2),
    }))
}

