#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct ValueIter {
    cur: Option<Box<ListNode>>,
}

impl Iterator for ValueIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.map(|cur| {
            let val = cur.val;
            self.cur = cur.next;
            val
        })
    }
}

pub struct NodeIter {
    cur: Option<Box<ListNode>>,
}

/*
 *impl<'a> IterMut for NodeIter<'a> {
 *    type Item = Box<ListNode>;
 *
 *    fn next(&mut self) -> Option<&mut Self::Item> {
 *        self.cur.map(|cur| {
 *        })
 *    }
 *}
 */

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
        }
    }

    //pub fn iter(&self) ->
}

type NodeOpt = Option<Box<ListNode>>;

fn construct_list(vals: &[i32]) -> NodeOpt {
    fn helper(mut iter: std::slice::Iter<i32>) -> NodeOpt {
        iter.next().map(|&val| {
            Box::new(ListNode {
                val,
                next: helper(iter),
            })
        })
    }
    helper(vals.iter())
}

fn list_length(head: &NodeOpt) -> usize {
    fn helper(head: &NodeOpt, n: usize) -> usize {
        match head {
            Some(head) => helper(&head.next, n + 1),
            None => n,
        }
    }
    helper(head, 0)
}

fn list_nth_mut(node: &mut NodeOpt, adv: usize) -> &mut NodeOpt {
    if adv > 0 {
        match node {
            Some(node) => list_nth_mut(&mut node.next, adv - 1),
            None => node,
        }
    } else {
        node
    }
}

fn list_reverse(mut head: NodeOpt) -> NodeOpt {
    let mut prev_opt: NodeOpt = None;
    let mut curr_opt: NodeOpt = head.take();
    while let Some(mut curr_node) = curr_opt.take() {
        let next_opt = curr_node.next.take();
        curr_node.next = prev_opt.take();
        prev_opt = Some(curr_node);
        curr_opt = next_opt;
    }
    prev_opt.take()
}

fn shuffle_list(l1: NodeOpt, l2: NodeOpt) -> NodeOpt {
    if let Some(mut l1_node) = l1 {
        if let Some(mut l2_node) = l2 {
            let l1_next = l1_node.next.take();
            let l2_next = l2_node.next.take();
            l2_node.next = shuffle_list(l1_next, l2_next);
            l1_node.next = Some(l2_node);
        }
        Some(l1_node)
    } else {
        None
    }
}

pub fn reorder_list(head: &mut NodeOpt) {
    let n = list_length(&head);
    if n > 2 {
        if let Some(head_node) = head {
            let mut l1 = head_node.next.take();
            let mid = list_nth_mut(&mut l1, (n - 3) / 2);
            let l2 = match mid {
                Some(mid_node) => mid_node.next.take(),
                None => unreachable!(),
            };
            head_node.next = shuffle_list(list_reverse(l2), l1);
        }
    }
}

fn main() {
    //let mut head = construct_list(&vec![]);
    //let mut head = construct_list(&vec![1]);
    //let mut head = construct_list(&vec![1, 2]);
    //let mut head = construct_list(&vec![1, 2, 3]);
    let mut head = construct_list(&vec![1, 2, 3, 4]);
    //let mut head = construct_list(&vec![1, 2, 3, 4, 5]);

    reorder_list(&mut head);
    println!("{:?}", head);
}
