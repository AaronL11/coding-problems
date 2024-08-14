type List = Option<Box<ListNode>>;

fn merge<'a>(l1: List, l2: List) -> List {
    let mut output = None;
    let mut next_node = &mut output;
    let mut l1_opt = l1;
    let mut l2_opt = l2;
    loop {
        let mut l1 = if let Some(x) = l1_opt {
            x
        } else {
            *next_node = l2_opt;
            break;
        };
        let mut l2 = if let Some(x) = l2_opt {
            x
        } else {
            *next_node = Some(l1);
            break;
        };
        if l1.val < l2.val {
            l1_opt = l1.next.take();
            l2_opt = Some(l2);
            *next_node = Some(l1);
        } else {
            l2_opt = l2.next.take();
            l1_opt = Some(l1);
            *next_node = Some(l2);
        }
        next_node = &mut next_node.as_mut().unwrap().next;
    }
    output
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<List>) -> List {
        let mut stack = lists;
        let mut stack2 = vec![];
        while let Some(l1) = stack.pop() {
            if let Some(l2) = stack.pop() {
                stack.push(merge(l1, l2))
            } else {
                stack2.push(l1)
            }
        }
        stack2.pop()?
    }
}
