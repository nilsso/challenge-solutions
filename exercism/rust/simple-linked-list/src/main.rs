use simple_linked_list::*;

fn main() {
    let mut l: SimpleLinkedList<i32> = SimpleLinkedList::new();

    l.push(1);
    l.push(2);

    println!("{:?}", l);

    println!("{:?}", l.pop());
    println!("{:?}", l);

    println!("{:?}", l.pop());
    println!("{:?}", l);
}
