use std::fmt::{Display, Debug};

#[derive(Clone, PartialEq, Debug)]
pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Box<Node<T>>>,
}

impl<'a, T> Iter<'a, T> {
    fn from(head: &'a Option<Box<Node<T>>>) -> Iter<'a, T> {
        Iter {
            next: head.as_ref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a Box<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref();
            node
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref(), }
    }

    pub fn nth_mut(&mut self, n: usize) -> Option<&mut Box<Node<T>>> {
        let mut node_opt = self.head.as_mut();
        for _ in 0..n {
            if let Some(node) = node_opt {
                node_opt = node.next.as_mut();
            }
        }
        node_opt
    }

    pub fn reverse(&mut self) {
        let mut prev_opt = None;
        let mut curr_opt = self.head.take();
        while let Some(mut curr_node) = curr_opt.take() {
            let next_opt = curr_node.next.take();
            curr_node.next = prev_opt.take();
            prev_opt = Some(curr_node);
            curr_opt = next_opt;
        }
        self.head = prev_opt.take();
    }

    pub fn interleave(&mut self, other: Option<Box<Node<T>>>) {
    }
}

impl<T> From<&[T]> for List<T>
where T: Copy + Display {
    fn from(vals: &[T]) -> List<T> {
        fn helper<T: Copy + Display>(mut iter: std::slice::Iter<T>) -> Option<Box<Node<T>>> {
            iter.next().map(|&val| Box::new(Node { val, next: helper(iter), }))
        }
        List { head: helper(vals.iter()) }
    }
}

impl<T: Display> std::fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.iter();
        if let Some(next) = iter.next() {
            write!(f, "{}", next.val)?;
        }
        while let Some(next) = iter.next() {
            write!(f, ", {}", next.val)?;
        }
        write!(f, "]")
    }
}

fn reorder<T: Display>(head: &mut Option<Box<Node<T>>>) {
    let mut l1 = List { head: head.take(), };

    let n = l1.iter().count() / 2;

    let mut l2: List<T> = List {
        head: l1.nth_mut(n).and_then(|mid| mid.next.take())
    };

    //l2.reverse();
    //l1.interleave(l2.head.take());

    println!("{}", l1);
    println!("{}", l2);

    //let mut l2 = l.iter_mut().nth(n - 1).unwrap().next.take();
}

fn main() {
    //let mut a = List::from([1].as_ref());
    //let mut a = List::from([1, 2].as_ref());
    let mut a = List::from([1, 2, 3].as_ref());
    //let mut a = List::from([1, 2, 3, 4].as_ref());
    //let mut a = List::from([1, 2, 3, 4, 5].as_ref());
    println!("{}", a);
    reorder(&mut a.head);
    println!("{}", a);
}

fn test<T: Copy + PartialEq + Display + Debug>(a: &[T], b: &[T]) {
    let (mut a, b) = (List::from(a), List::from(b));
    println!(" original {}", a);
    reorder(&mut a.head);
    println!("reordered {}", a);
    println!(" expected {}", b);
    assert_eq!(a, b);
}

#[test]
fn test_1() {
    test(&[1], &[1]);
}

#[test]
fn test_1_2() {
    test(&[1, 2], &[1, 2]);
}

#[test]
#[ignore]
fn test_1_2_3() {
    test(&[1, 2, 3], &[1, 3, 2]);
}
